#![allow(dead_code)]

use std::path::Path;

use chrono::{DateTime, SecondsFormat, Utc};
use rusqlite::{params, Connection};
use thiserror::Error;
use uuid::Uuid;

use crate::kernel::capability::CapabilityInvocation;
use crate::kernel::deepseek::DeepSeekChatTelemetry;
use crate::kernel::models::{
    KernelEvent, MemoryCandidate, MemoryCandidateMergePreview, MemoryCandidateRecord,
    MemoryCandidateReplacePreview, MemoryCandidateResolution, MemoryCandidateSource,
    MemoryCandidateStatus, MemoryConflictSummary, MemoryRecord, MemoryRecordDeletion,
    MemoryRecordLink, MemoryRecordLinkSummary, MemoryRecordUpdate, TaskRecord,
};
use crate::kernel::policy::{
    capability_risk, CapabilityAccessRecord, CapabilityAccessRequest, CapabilityAccessStatus,
    CapabilityGrantState, CapabilityKind, PermissionAuditEntry, PermissionResolution, RiskLevel,
};
use crate::kernel::work_package::{
    WorkPackage, WorkPackageImportPreview, WorkPackageImportSummary,
    WorkPackageMemoryCandidateImportPreview, WorkPackageMemoryCandidateImportSummary,
    WorkPackageOperationsBriefingImportPreview, WorkPackageOperationsBriefingImportSummary,
    WorkPackageTaskImportPreview, WorkPackageWorkflowTemplateImportPreview,
    WorkPackageWorkflowTemplateImportSummary,
};
use crate::kernel::workflow::{OperationsBriefingRun, WorkflowTemplatePackage};

pub const CAPABILITY_ACCESS_REQUESTED_EVENT: &str = "capability_access.requested";
pub const CAPABILITY_INVOCATION_RECORDED_EVENT: &str = "capability_invocation.recorded";
pub const DEEPSEEK_CHAT_TELEMETRY_RECORDED_EVENT: &str = "deepseek_chat.telemetry_recorded";
pub const MEMORY_CANDIDATE_PROPOSED_EVENT: &str = "memory_candidate.proposed";
pub const MEMORY_CANDIDATE_RESOLVED_EVENT: &str = "memory_candidate.resolved";
pub const MEMORY_RECORD_CREATED_EVENT: &str = "memory_record.created";
pub const MEMORY_RECORD_UPDATED_EVENT: &str = "memory_record.updated";
pub const MEMORY_RECORD_DELETED_EVENT: &str = "memory_record.deleted";
pub const MEMORY_RECORD_LINKED_EVENT: &str = "memory_record.linked";
pub const OPERATIONS_BRIEFING_RUN_RECORDED_EVENT: &str = "operations_briefing.run_recorded";
pub const PERMISSION_AUDIT_RECORDED_EVENT: &str = "permission_audit.recorded";
pub const PERMISSION_RESOLUTION_RECORDED_EVENT: &str = "permission_resolution.recorded";
pub const TASK_RECORD_CREATED_EVENT: &str = "task_record.created";
pub const WORKFLOW_TEMPLATE_PACKAGE_IMPORTED_EVENT: &str = "workflow_template_package.imported";

#[derive(Debug, Error)]
pub enum EventStoreError {
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("timestamp parse error: {0}")]
    Timestamp(#[from] chrono::ParseError),

    #[error("uuid parse error: {0}")]
    Uuid(#[from] uuid::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("invalid state: {0}")]
    InvalidState(String),
}

pub type EventStoreResult<T> = Result<T, EventStoreError>;

pub struct EventStore {
    conn: Connection,
}

impl EventStore {
    pub fn open(path: impl AsRef<Path>) -> EventStoreResult<Self> {
        let store = Self {
            conn: Connection::open(path)?,
        };
        store.migrate()?;
        Ok(store)
    }

    pub fn open_memory() -> EventStoreResult<Self> {
        let store = Self {
            conn: Connection::open_in_memory()?,
        };
        store.migrate()?;
        Ok(store)
    }

    fn migrate(&self) -> EventStoreResult<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS kernel_events (
                id TEXT PRIMARY KEY NOT NULL,
                event_type TEXT NOT NULL,
                payload_json TEXT NOT NULL,
                created_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_kernel_events_created_at
                ON kernel_events (created_at);
            "#,
        )?;
        Ok(())
    }

    pub fn append(&self, event: &KernelEvent) -> EventStoreResult<()> {
        self.conn.execute(
            r#"
            INSERT INTO kernel_events (id, event_type, payload_json, created_at)
            VALUES (?1, ?2, ?3, ?4)
            "#,
            params![
                event.id.to_string(),
                event.event_type,
                event.payload_json,
                event.created_at.to_rfc3339_opts(SecondsFormat::Nanos, true),
            ],
        )?;
        Ok(())
    }

    pub fn list_recent(&self, limit: usize) -> EventStoreResult<Vec<KernelEvent>> {
        let limit = i64::try_from(limit).unwrap_or(i64::MAX);
        let mut statement = self.conn.prepare(
            r#"
            SELECT id, event_type, payload_json, created_at
            FROM kernel_events
            ORDER BY created_at DESC
            LIMIT ?1
            "#,
        )?;
        let rows = statement
            .query_map(params![limit], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut events = Vec::with_capacity(rows.len());
        for (id, event_type, payload_json, created_at) in rows {
            events.push(KernelEvent {
                id: Uuid::parse_str(&id)?,
                event_type,
                payload_json,
                created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
            });
        }

        Ok(events)
    }

    pub fn append_task_record(&self, record: &TaskRecord) -> EventStoreResult<()> {
        let event = KernelEvent::new(TASK_RECORD_CREATED_EVENT, record)?;
        self.append(&event)
    }

    pub fn list_task_records(&self) -> EventStoreResult<Vec<TaskRecord>> {
        let events = self.list_by_type(TASK_RECORD_CREATED_EVENT, 500)?;
        events
            .into_iter()
            .map(|event| {
                serde_json::from_str::<TaskRecord>(&event.payload_json).map_err(Into::into)
            })
            .collect()
    }

    pub fn import_task_records(
        &self,
        records: &[TaskRecord],
    ) -> EventStoreResult<WorkPackageImportSummary> {
        let mut existing_ids = self
            .list_task_records()?
            .into_iter()
            .map(|record| record.id)
            .collect::<std::collections::HashSet<_>>();
        let mut summary = WorkPackageImportSummary {
            imported: 0,
            skipped: 0,
            memory_candidates: WorkPackageMemoryCandidateImportSummary {
                imported: 0,
                skipped: 0,
            },
            operations_briefing_runs: WorkPackageOperationsBriefingImportSummary {
                imported: 0,
                skipped: 0,
            },
            workflow_templates: WorkPackageWorkflowTemplateImportSummary {
                imported: 0,
                skipped: 0,
            },
        };

        for record in records {
            if existing_ids.contains(&record.id) {
                summary.skipped += 1;
                continue;
            }

            self.append_task_record(record)?;
            let memory = MemoryRecord::from_task_record(record);
            self.append_memory_record(&memory)?;
            existing_ids.insert(record.id);
            summary.imported += 1;
        }

        Ok(summary)
    }

    pub fn preview_work_package_import(
        &self,
        package: &WorkPackage,
    ) -> EventStoreResult<WorkPackageImportPreview> {
        let existing_ids = self
            .list_task_records()?
            .into_iter()
            .map(|record| record.id)
            .collect::<std::collections::HashSet<_>>();
        let skipped = package
            .task_records
            .iter()
            .filter(|record| existing_ids.contains(&record.id))
            .count();
        let total = package.task_records.len();
        let existing_candidate_ids = self
            .list_memory_candidates()?
            .into_iter()
            .map(|candidate| candidate.id)
            .collect::<std::collections::HashSet<_>>();
        let skipped_candidates = package
            .memory_candidates
            .iter()
            .filter(|candidate| existing_candidate_ids.contains(&candidate.id))
            .count();
        let total_candidates = package.memory_candidates.len();
        let existing_template_ids = self
            .list_workflow_template_packages()?
            .into_iter()
            .map(|template| template.id)
            .collect::<std::collections::HashSet<_>>();
        let skipped_templates = package
            .workflow_templates
            .iter()
            .filter(|template| existing_template_ids.contains(&template.id))
            .count();
        let total_templates = package.workflow_templates.len();

        Ok(WorkPackageImportPreview {
            task_records: WorkPackageTaskImportPreview {
                total,
                new: total.saturating_sub(skipped),
                skipped,
            },
            memory_candidates: WorkPackageMemoryCandidateImportPreview {
                total: total_candidates,
                new: total_candidates.saturating_sub(skipped_candidates),
                skipped: skipped_candidates,
                review_supported: true,
            },
            operations_briefing_runs: WorkPackageOperationsBriefingImportPreview {
                total: package.operations_briefing_runs.len(),
                replay_supported: true,
            },
            workflow_templates: WorkPackageWorkflowTemplateImportPreview {
                total: total_templates,
                new: total_templates.saturating_sub(skipped_templates),
                skipped: skipped_templates,
                import_supported: true,
            },
        })
    }

    pub fn append_workflow_template_package(
        &self,
        template: &WorkflowTemplatePackage,
    ) -> EventStoreResult<bool> {
        let existing = self
            .list_workflow_template_packages()?
            .into_iter()
            .any(|local_template| local_template.id == template.id);
        if existing {
            return Ok(false);
        }

        let event = KernelEvent::new(WORKFLOW_TEMPLATE_PACKAGE_IMPORTED_EVENT, template)?;
        self.append(&event)?;
        Ok(true)
    }

    pub fn list_workflow_template_packages(
        &self,
    ) -> EventStoreResult<Vec<WorkflowTemplatePackage>> {
        let events = self.list_by_type(WORKFLOW_TEMPLATE_PACKAGE_IMPORTED_EVENT, 500)?;
        events
            .into_iter()
            .map(|event| {
                serde_json::from_str::<WorkflowTemplatePackage>(&event.payload_json)
                    .map_err(Into::into)
            })
            .collect()
    }

    pub fn import_workflow_template_packages(
        &self,
        templates: &[WorkflowTemplatePackage],
    ) -> EventStoreResult<WorkPackageWorkflowTemplateImportSummary> {
        let mut summary = WorkPackageWorkflowTemplateImportSummary {
            imported: 0,
            skipped: 0,
        };

        for template in templates {
            if self.append_workflow_template_package(template)? {
                summary.imported += 1;
            } else {
                summary.skipped += 1;
            }
        }

        Ok(summary)
    }

    pub fn append_memory_record(&self, record: &MemoryRecord) -> EventStoreResult<bool> {
        if let Some(source_id) = record.source_id {
            let existing = self.list_memory_records()?.into_iter().any(|memory| {
                memory.source == record.source && memory.source_id == Some(source_id)
            });
            if existing {
                return Ok(false);
            }
        }

        let event = KernelEvent::new(MEMORY_RECORD_CREATED_EVENT, record)?;
        self.append(&event)?;
        Ok(true)
    }

    pub fn list_memory_records(&self) -> EventStoreResult<Vec<MemoryRecord>> {
        self.list_memory_records_at(Utc::now())
    }

    pub fn list_memory_records_at(
        &self,
        now: DateTime<Utc>,
    ) -> EventStoreResult<Vec<MemoryRecord>> {
        let deleted_memory_ids = self
            .list_memory_record_deletions()?
            .into_iter()
            .map(|deletion| deletion.memory_id)
            .collect::<std::collections::HashSet<_>>();
        let latest_updates = self.list_memory_record_updates()?.into_iter().fold(
            std::collections::HashMap::new(),
            |mut updates, update| {
                updates.entry(update.memory_id).or_insert(update);
                updates
            },
        );
        let events = self.list_by_type(MEMORY_RECORD_CREATED_EVENT, 500)?;
        let memories = events
            .into_iter()
            .map(|event| {
                serde_json::from_str::<MemoryRecord>(&event.payload_json).map_err(Into::into)
            })
            .map(|record| {
                record.map(|memory| {
                    latest_updates
                        .get(&memory.id)
                        .map(|update| update.apply_to(&memory))
                        .unwrap_or(memory)
                })
            })
            .filter(|record| {
                record
                    .as_ref()
                    .map(|memory| !deleted_memory_ids.contains(&memory.id))
                    .unwrap_or(true)
            })
            .filter(|record| {
                record
                    .as_ref()
                    .map(|memory| !memory.is_expired_at(now))
                    .unwrap_or(true)
            })
            .collect::<EventStoreResult<Vec<_>>>()?;

        self.with_memory_record_links(memories)
    }

    fn with_memory_record_links(
        &self,
        memories: Vec<MemoryRecord>,
    ) -> EventStoreResult<Vec<MemoryRecord>> {
        let visible_memory_ids = memories
            .iter()
            .map(|memory| memory.id)
            .collect::<std::collections::HashSet<_>>();
        let summaries_by_id = memories
            .iter()
            .map(|memory| (memory.id, MemoryRecordLinkSummary::from(memory)))
            .collect::<std::collections::HashMap<_, _>>();
        let mut linked_ids_by_memory_id: std::collections::HashMap<Uuid, Vec<Uuid>> =
            std::collections::HashMap::new();

        for link in self.list_memory_record_links()? {
            if !visible_memory_ids.contains(&link.source_memory_id)
                || !visible_memory_ids.contains(&link.target_memory_id)
            {
                continue;
            }

            push_unique_link(
                &mut linked_ids_by_memory_id,
                link.source_memory_id,
                link.target_memory_id,
            );
            push_unique_link(
                &mut linked_ids_by_memory_id,
                link.target_memory_id,
                link.source_memory_id,
            );
        }

        Ok(memories
            .into_iter()
            .map(|mut memory| {
                let linked_memory_ids = linked_ids_by_memory_id
                    .remove(&memory.id)
                    .unwrap_or_default();
                let linked_memories = linked_memory_ids
                    .iter()
                    .filter_map(|id| summaries_by_id.get(id).cloned())
                    .collect();
                memory.linked_memory_ids = linked_memory_ids;
                memory.linked_memories = linked_memories;
                memory
            })
            .collect())
    }

    pub fn list_memory_record_updates(&self) -> EventStoreResult<Vec<MemoryRecordUpdate>> {
        let events = self.list_by_type(MEMORY_RECORD_UPDATED_EVENT, 500)?;
        events
            .into_iter()
            .map(|event| {
                serde_json::from_str::<MemoryRecordUpdate>(&event.payload_json).map_err(Into::into)
            })
            .collect()
    }

    pub fn update_memory_record(
        &self,
        memory_id: Uuid,
        title: String,
        body: String,
        memory_type: crate::kernel::models::MemoryType,
        scope: crate::kernel::models::MemoryScope,
        sensitivity: crate::kernel::models::MemorySensitivity,
        lifecycle: crate::kernel::models::MemoryLifecycle,
        expires_at: Option<DateTime<Utc>>,
        note: String,
    ) -> EventStoreResult<MemoryRecordUpdate> {
        let existing = self
            .list_memory_records()?
            .into_iter()
            .find(|memory| memory.id == memory_id)
            .ok_or_else(|| {
                EventStoreError::NotFound(format!("memory record {memory_id} was not found"))
            })?;
        let update = MemoryRecordUpdate::new(
            memory_id,
            title,
            body,
            memory_type,
            scope,
            sensitivity,
            lifecycle,
            existing.pinned,
            expires_at,
            note,
        )
        .map_err(EventStoreError::InvalidState)?;
        let event = KernelEvent::new(MEMORY_RECORD_UPDATED_EVENT, &update)?;
        self.append(&event)?;
        Ok(update)
    }

    pub fn list_memory_record_deletions(&self) -> EventStoreResult<Vec<MemoryRecordDeletion>> {
        let events = self.list_by_type(MEMORY_RECORD_DELETED_EVENT, 500)?;
        events
            .into_iter()
            .map(|event| {
                serde_json::from_str::<MemoryRecordDeletion>(&event.payload_json)
                    .map_err(Into::into)
            })
            .collect()
    }

    pub fn delete_memory_record(
        &self,
        memory_id: Uuid,
        note: String,
    ) -> EventStoreResult<MemoryRecordDeletion> {
        let exists = self
            .list_memory_records()?
            .into_iter()
            .any(|memory| memory.id == memory_id);
        if !exists {
            return Err(EventStoreError::NotFound(format!(
                "memory record {memory_id} was not found"
            )));
        }

        let deletion = MemoryRecordDeletion::new(memory_id, note);
        let event = KernelEvent::new(MEMORY_RECORD_DELETED_EVENT, &deletion)?;
        self.append(&event)?;
        Ok(deletion)
    }

    pub fn append_memory_record_link(&self, link: &MemoryRecordLink) -> EventStoreResult<()> {
        let event = KernelEvent::new(MEMORY_RECORD_LINKED_EVENT, link)?;
        self.append(&event)
    }

    pub fn list_memory_record_links(&self) -> EventStoreResult<Vec<MemoryRecordLink>> {
        let events = self.list_by_type(MEMORY_RECORD_LINKED_EVENT, 500)?;
        events
            .into_iter()
            .map(|event| {
                serde_json::from_str::<MemoryRecordLink>(&event.payload_json).map_err(Into::into)
            })
            .collect()
    }

    pub fn search_memory_records(&self, query: &str) -> EventStoreResult<Vec<MemoryRecord>> {
        self.search_memory_records_at(query, Utc::now())
    }

    pub fn search_memory_records_at(
        &self,
        query: &str,
        now: DateTime<Utc>,
    ) -> EventStoreResult<Vec<MemoryRecord>> {
        let query = query.trim().to_lowercase();
        let memories = self.list_memory_records_at(now)?;
        if query.is_empty() {
            return Ok(memories);
        }

        Ok(memories
            .into_iter()
            .filter(|memory| {
                memory.title.to_lowercase().contains(&query)
                    || memory.body.to_lowercase().contains(&query)
            })
            .collect())
    }

    pub fn append_memory_candidate(&self, candidate: &MemoryCandidate) -> EventStoreResult<()> {
        let event = KernelEvent::new(MEMORY_CANDIDATE_PROPOSED_EVENT, candidate)?;
        self.append(&event)
    }

    pub fn import_memory_candidates(
        &self,
        candidates: &[MemoryCandidate],
    ) -> EventStoreResult<WorkPackageMemoryCandidateImportSummary> {
        let mut existing_ids = self
            .list_memory_candidates()?
            .into_iter()
            .map(|candidate| candidate.id)
            .collect::<std::collections::HashSet<_>>();
        let mut summary = WorkPackageMemoryCandidateImportSummary {
            imported: 0,
            skipped: 0,
        };

        for candidate in candidates {
            if existing_ids.contains(&candidate.id) {
                summary.skipped += 1;
                continue;
            }

            let mut imported_candidate = candidate.clone();
            imported_candidate.source = MemoryCandidateSource::Import;
            self.append_memory_candidate(&imported_candidate)?;
            existing_ids.insert(imported_candidate.id);
            summary.imported += 1;
        }

        Ok(summary)
    }

    pub fn list_memory_candidates(&self) -> EventStoreResult<Vec<MemoryCandidate>> {
        let events = self.list_by_type(MEMORY_CANDIDATE_PROPOSED_EVENT, 500)?;
        events
            .into_iter()
            .map(|event| {
                serde_json::from_str::<MemoryCandidate>(&event.payload_json).map_err(Into::into)
            })
            .collect()
    }

    pub fn append_memory_candidate_resolution(
        &self,
        resolution: &MemoryCandidateResolution,
    ) -> EventStoreResult<()> {
        let event = KernelEvent::new(MEMORY_CANDIDATE_RESOLVED_EVENT, resolution)?;
        self.append(&event)
    }

    pub fn list_memory_candidate_resolutions(
        &self,
    ) -> EventStoreResult<Vec<MemoryCandidateResolution>> {
        let events = self.list_by_type(MEMORY_CANDIDATE_RESOLVED_EVENT, 500)?;
        events
            .into_iter()
            .map(|event| {
                serde_json::from_str::<MemoryCandidateResolution>(&event.payload_json)
                    .map_err(Into::into)
            })
            .collect()
    }

    pub fn list_memory_candidate_records(&self) -> EventStoreResult<Vec<MemoryCandidateRecord>> {
        let mut latest_resolution_by_candidate_id = std::collections::HashMap::new();
        for resolution in self.list_memory_candidate_resolutions()? {
            latest_resolution_by_candidate_id
                .entry(resolution.candidate_id)
                .or_insert(resolution);
        }
        let visible_memories = self.list_memory_records()?;

        self.list_memory_candidates()?
            .into_iter()
            .map(|candidate| {
                let resolution = latest_resolution_by_candidate_id
                    .remove(&candidate.id)
                    .map(|resolution| resolution.to_owned());
                let effective_status = match &resolution {
                    Some(resolution) if resolution.accepted => MemoryCandidateStatus::Accepted,
                    Some(_) => MemoryCandidateStatus::Rejected,
                    None => MemoryCandidateStatus::Pending,
                };
                let conflicting_memories = visible_memories
                    .iter()
                    .filter(|memory| memory_candidate_conflicts_with_record(&candidate, memory))
                    .map(MemoryConflictSummary::from)
                    .collect::<Vec<_>>();
                let conflicting_memory_ids = conflicting_memories
                    .iter()
                    .map(|memory| memory.id)
                    .collect();

                Ok(MemoryCandidateRecord {
                    candidate,
                    resolution,
                    effective_status,
                    conflicting_memory_ids,
                    conflicting_memories,
                })
            })
            .collect()
    }

    pub fn resolve_memory_candidate(
        &self,
        candidate_id: Uuid,
        accepted: bool,
        note: String,
    ) -> EventStoreResult<MemoryCandidateResolution> {
        let record = self
            .list_memory_candidate_records()?
            .into_iter()
            .find(|record| record.candidate.id == candidate_id)
            .ok_or_else(|| {
                EventStoreError::NotFound("memory candidate does not exist".to_string())
            })?;

        if record.resolution.is_some() {
            return Err(EventStoreError::InvalidState(
                "memory candidate is already resolved".to_string(),
            ));
        }

        let resolution = MemoryCandidateResolution::new(candidate_id, accepted, note);
        self.append_memory_candidate_resolution(&resolution)?;
        if accepted {
            let memory = MemoryRecord::from_memory_candidate(&record.candidate);
            self.append_memory_record(&memory)?;
        }
        Ok(resolution)
    }

    pub fn preview_memory_candidate_merge(
        &self,
        candidate_id: Uuid,
        source_memory_ids: Vec<Uuid>,
    ) -> EventStoreResult<MemoryCandidateMergePreview> {
        let mut unique_source_memory_ids = Vec::new();
        let mut seen_source_memory_ids = std::collections::HashSet::new();
        for memory_id in source_memory_ids {
            if seen_source_memory_ids.insert(memory_id) {
                unique_source_memory_ids.push(memory_id);
            }
        }
        if unique_source_memory_ids.is_empty() {
            return Err(EventStoreError::InvalidState(
                "memory candidate merge preview requires at least one source memory".to_string(),
            ));
        }

        let record = self
            .list_memory_candidate_records()?
            .into_iter()
            .find(|record| record.candidate.id == candidate_id)
            .ok_or_else(|| {
                EventStoreError::NotFound("memory candidate does not exist".to_string())
            })?;

        if record.resolution.is_some() {
            return Err(EventStoreError::InvalidState(
                "memory candidate is already resolved".to_string(),
            ));
        }

        let visible_memories = self.list_memory_records()?;
        let visible_memories_by_id = visible_memories
            .iter()
            .map(|memory| (memory.id, memory))
            .collect::<std::collections::HashMap<_, _>>();
        let mut source_bodies = Vec::new();
        for memory_id in &unique_source_memory_ids {
            let memory = visible_memories_by_id.get(memory_id).ok_or_else(|| {
                EventStoreError::NotFound(format!("memory record {memory_id} was not found"))
            })?;
            push_unique_memory_body(&mut source_bodies, &memory.body);
        }
        push_unique_memory_body(&mut source_bodies, &record.candidate.body);

        Ok(MemoryCandidateMergePreview {
            candidate_id,
            source_memory_ids: unique_source_memory_ids,
            title: record.candidate.title,
            body: source_bodies.join("\n\n"),
            memory_type: record.candidate.memory_type,
            scope: record.candidate.scope,
            sensitivity: record.candidate.sensitivity,
            lifecycle: record.candidate.lifecycle,
            expires_at: record.candidate.expires_at,
        })
    }

    pub fn preview_memory_candidate_replace(
        &self,
        candidate_id: Uuid,
        target_memory_ids: Vec<Uuid>,
    ) -> EventStoreResult<MemoryCandidateReplacePreview> {
        let mut unique_target_memory_ids = Vec::new();
        let mut seen_target_memory_ids = std::collections::HashSet::new();
        for memory_id in target_memory_ids {
            if seen_target_memory_ids.insert(memory_id) {
                unique_target_memory_ids.push(memory_id);
            }
        }
        if unique_target_memory_ids.is_empty() {
            return Err(EventStoreError::InvalidState(
                "memory candidate replace preview requires at least one target memory".to_string(),
            ));
        }

        let record = self
            .list_memory_candidate_records()?
            .into_iter()
            .find(|record| record.candidate.id == candidate_id)
            .ok_or_else(|| {
                EventStoreError::NotFound("memory candidate does not exist".to_string())
            })?;

        if record.resolution.is_some() {
            return Err(EventStoreError::InvalidState(
                "memory candidate is already resolved".to_string(),
            ));
        }

        let visible_memories = self.list_memory_records()?;
        let visible_memories_by_id = visible_memories
            .iter()
            .map(|memory| (memory.id, memory))
            .collect::<std::collections::HashMap<_, _>>();
        let mut target_memories = Vec::new();
        for memory_id in &unique_target_memory_ids {
            let memory = visible_memories_by_id.get(memory_id).ok_or_else(|| {
                EventStoreError::NotFound(format!("memory record {memory_id} was not found"))
            })?;
            target_memories.push(MemoryConflictSummary::from(*memory));
        }

        Ok(MemoryCandidateReplacePreview {
            candidate_id,
            target_memory_ids: unique_target_memory_ids,
            replacement_title: record.candidate.title,
            replacement_body: record.candidate.body,
            memory_type: record.candidate.memory_type,
            scope: record.candidate.scope,
            sensitivity: record.candidate.sensitivity,
            lifecycle: record.candidate.lifecycle,
            expires_at: record.candidate.expires_at,
            target_memories,
        })
    }

    pub fn merge_memory_candidate_with_conflicts(
        &self,
        candidate_id: Uuid,
        source_memory_ids: Vec<Uuid>,
        note: String,
    ) -> EventStoreResult<MemoryCandidateResolution> {
        let preview = self.preview_memory_candidate_merge(candidate_id, source_memory_ids)?;
        let record = self
            .list_memory_candidate_records()?
            .into_iter()
            .find(|record| record.candidate.id == candidate_id)
            .ok_or_else(|| {
                EventStoreError::NotFound("memory candidate does not exist".to_string())
            })?;
        if record.resolution.is_some() {
            return Err(EventStoreError::InvalidState(
                "memory candidate is already resolved".to_string(),
            ));
        }

        let resolution = MemoryCandidateResolution::new(candidate_id, true, note.clone());
        self.append_memory_candidate_resolution(&resolution)?;

        let mut merged_candidate = record.candidate.clone();
        merged_candidate.title = preview.title;
        merged_candidate.body = preview.body;
        merged_candidate.memory_type = preview.memory_type;
        merged_candidate.scope = preview.scope;
        merged_candidate.sensitivity = preview.sensitivity;
        merged_candidate.lifecycle = preview.lifecycle;
        merged_candidate.expires_at = preview.expires_at;
        let merged_memory = MemoryRecord::from_memory_candidate(&merged_candidate);
        if !self.append_memory_record(&merged_memory)? {
            return Err(EventStoreError::InvalidState(
                "accepted memory candidate was already written".to_string(),
            ));
        }

        for memory_id in preview.source_memory_ids {
            let link = MemoryRecordLink::new(
                merged_memory.id,
                memory_id,
                Some(candidate_id),
                note.clone(),
            )
            .map_err(EventStoreError::InvalidState)?;
            self.append_memory_record_link(&link)?;
            self.delete_memory_record(memory_id, note.clone())?;
        }

        Ok(resolution)
    }

    pub fn replace_memory_candidate_conflicts(
        &self,
        candidate_id: Uuid,
        target_memory_ids: Vec<Uuid>,
        note: String,
    ) -> EventStoreResult<MemoryCandidateResolution> {
        let preview = self.preview_memory_candidate_replace(candidate_id, target_memory_ids)?;
        let record = self
            .list_memory_candidate_records()?
            .into_iter()
            .find(|record| record.candidate.id == candidate_id)
            .ok_or_else(|| {
                EventStoreError::NotFound("memory candidate does not exist".to_string())
            })?;
        if record.resolution.is_some() {
            return Err(EventStoreError::InvalidState(
                "memory candidate is already resolved".to_string(),
            ));
        }

        let resolution = MemoryCandidateResolution::new(candidate_id, true, note.clone());
        self.append_memory_candidate_resolution(&resolution)?;

        let replacement_memory = MemoryRecord::from_memory_candidate(&record.candidate);
        if !self.append_memory_record(&replacement_memory)? {
            return Err(EventStoreError::InvalidState(
                "accepted memory candidate was already written".to_string(),
            ));
        }

        for memory_id in preview.target_memory_ids {
            let link = MemoryRecordLink::new(
                replacement_memory.id,
                memory_id,
                Some(candidate_id),
                note.clone(),
            )
            .map_err(EventStoreError::InvalidState)?;
            self.append_memory_record_link(&link)?;
            self.delete_memory_record(memory_id, note.clone())?;
        }

        Ok(resolution)
    }

    pub fn link_memory_candidate_to_conflicts(
        &self,
        candidate_id: Uuid,
        linked_memory_ids: Vec<Uuid>,
        note: String,
    ) -> EventStoreResult<MemoryCandidateResolution> {
        let mut unique_linked_memory_ids = Vec::new();
        let mut seen_linked_memory_ids = std::collections::HashSet::new();
        for memory_id in linked_memory_ids {
            if seen_linked_memory_ids.insert(memory_id) {
                unique_linked_memory_ids.push(memory_id);
            }
        }
        if unique_linked_memory_ids.is_empty() {
            return Err(EventStoreError::InvalidState(
                "memory candidate link requires at least one target memory".to_string(),
            ));
        }

        let record = self
            .list_memory_candidate_records()?
            .into_iter()
            .find(|record| record.candidate.id == candidate_id)
            .ok_or_else(|| {
                EventStoreError::NotFound("memory candidate does not exist".to_string())
            })?;

        if record.resolution.is_some() {
            return Err(EventStoreError::InvalidState(
                "memory candidate is already resolved".to_string(),
            ));
        }

        let visible_memory_ids = self
            .list_memory_records()?
            .into_iter()
            .map(|memory| memory.id)
            .collect::<std::collections::HashSet<_>>();
        for memory_id in &unique_linked_memory_ids {
            if !visible_memory_ids.contains(memory_id) {
                return Err(EventStoreError::NotFound(format!(
                    "memory record {memory_id} was not found"
                )));
            }
        }

        let resolution = MemoryCandidateResolution::new(candidate_id, true, note.clone());
        self.append_memory_candidate_resolution(&resolution)?;
        let memory = MemoryRecord::from_memory_candidate(&record.candidate);
        self.append_memory_record(&memory)?;

        for linked_memory_id in unique_linked_memory_ids {
            let link = MemoryRecordLink::new(
                memory.id,
                linked_memory_id,
                Some(candidate_id),
                note.clone(),
            )
            .map_err(EventStoreError::InvalidState)?;
            self.append_memory_record_link(&link)?;
        }

        Ok(resolution)
    }

    pub fn append_permission_audit_entry(
        &self,
        entry: &PermissionAuditEntry,
    ) -> EventStoreResult<()> {
        let event = KernelEvent::new(PERMISSION_AUDIT_RECORDED_EVENT, entry)?;
        self.append(&event)
    }

    pub fn append_deepseek_chat_telemetry(
        &self,
        telemetry: &DeepSeekChatTelemetry,
    ) -> EventStoreResult<()> {
        let event = KernelEvent::new(DEEPSEEK_CHAT_TELEMETRY_RECORDED_EVENT, telemetry)?;
        self.append(&event)
    }

    pub fn list_deepseek_chat_telemetry(&self) -> EventStoreResult<Vec<DeepSeekChatTelemetry>> {
        let events = self.list_by_type(DEEPSEEK_CHAT_TELEMETRY_RECORDED_EVENT, 500)?;
        events
            .into_iter()
            .map(|event| {
                serde_json::from_str::<DeepSeekChatTelemetry>(&event.payload_json)
                    .map_err(Into::into)
            })
            .collect()
    }

    pub fn list_permission_audit_entries(&self) -> EventStoreResult<Vec<PermissionAuditEntry>> {
        let events = self.list_by_type(PERMISSION_AUDIT_RECORDED_EVENT, 100)?;
        events
            .into_iter()
            .map(|event| {
                serde_json::from_str::<PermissionAuditEntry>(&event.payload_json)
                    .map_err(Into::into)
            })
            .collect()
    }

    pub fn append_capability_access_request(
        &self,
        request: &CapabilityAccessRequest,
    ) -> EventStoreResult<()> {
        let event = KernelEvent::new(CAPABILITY_ACCESS_REQUESTED_EVENT, request)?;
        self.append(&event)
    }

    pub fn list_capability_access_requests(
        &self,
    ) -> EventStoreResult<Vec<CapabilityAccessRequest>> {
        let events = self.list_by_type(CAPABILITY_ACCESS_REQUESTED_EVENT, 200)?;
        events
            .into_iter()
            .map(|event| {
                serde_json::from_str::<CapabilityAccessRequest>(&event.payload_json)
                    .map_err(Into::into)
            })
            .collect()
    }

    pub fn append_permission_resolution(
        &self,
        resolution: &PermissionResolution,
    ) -> EventStoreResult<()> {
        let event = KernelEvent::new(PERMISSION_RESOLUTION_RECORDED_EVENT, resolution)?;
        self.append(&event)
    }

    pub fn list_permission_resolutions(&self) -> EventStoreResult<Vec<PermissionResolution>> {
        let events = self.list_by_type(PERMISSION_RESOLUTION_RECORDED_EVENT, 200)?;
        events
            .into_iter()
            .map(|event| {
                serde_json::from_str::<PermissionResolution>(&event.payload_json)
                    .map_err(Into::into)
            })
            .collect()
    }

    pub fn list_capability_access_records(&self) -> EventStoreResult<Vec<CapabilityAccessRecord>> {
        let mut latest_resolution_by_request_id = std::collections::HashMap::new();
        for resolution in self.list_permission_resolutions()? {
            latest_resolution_by_request_id
                .entry(resolution.request_id)
                .or_insert(resolution);
        }
        let invocations = self.list_capability_invocations()?;

        self.list_capability_access_requests()?
            .into_iter()
            .map(|request| {
                let resolution = latest_resolution_by_request_id
                    .remove(&request.id)
                    .map(|resolution| resolution.to_owned());
                let effective_status = match &resolution {
                    Some(resolution) if resolution.approved => CapabilityAccessStatus::Approved,
                    Some(_) => CapabilityAccessStatus::Rejected,
                    None => request.status,
                };
                let grant_state = capability_grant_state(
                    &request,
                    resolution.as_ref(),
                    effective_status,
                    &invocations,
                );

                Ok(CapabilityAccessRecord {
                    request,
                    resolution,
                    effective_status,
                    grant_state,
                })
            })
            .collect()
    }

    pub fn list_pending_capability_access_records(
        &self,
    ) -> EventStoreResult<Vec<CapabilityAccessRecord>> {
        Ok(self
            .list_capability_access_records()?
            .into_iter()
            .filter(|record| record.effective_status == CapabilityAccessStatus::PendingApproval)
            .collect())
    }

    pub fn has_user_approved_capability(
        &self,
        capability: CapabilityKind,
    ) -> EventStoreResult<bool> {
        Ok(self
            .available_capability_grant_request_id(capability)?
            .is_some())
    }

    pub fn available_capability_grant_request_id(
        &self,
        capability: CapabilityKind,
    ) -> EventStoreResult<Option<Uuid>> {
        Ok(self
            .list_capability_access_records()?
            .into_iter()
            .find(|record| {
                record.request.capability == capability
                    && matches!(
                        record.grant_state,
                        CapabilityGrantState::Reusable | CapabilityGrantState::OneShotAvailable
                    )
            })
            .map(|record| record.request.id))
    }

    pub fn resolve_capability_access_request(
        &self,
        request_id: Uuid,
        approved: bool,
        note: String,
    ) -> EventStoreResult<PermissionResolution> {
        let record = self
            .list_capability_access_records()?
            .into_iter()
            .find(|record| record.request.id == request_id)
            .ok_or_else(|| {
                EventStoreError::NotFound("capability access request does not exist".to_string())
            })?;

        if record.resolution.is_some() {
            return Err(EventStoreError::InvalidState(
                "capability access request is already resolved".to_string(),
            ));
        }

        if record.effective_status != CapabilityAccessStatus::PendingApproval {
            return Err(EventStoreError::InvalidState(
                "capability access request does not require approval".to_string(),
            ));
        }

        let resolution = PermissionResolution::new(request_id, approved, note);
        self.append_permission_resolution(&resolution)?;
        Ok(resolution)
    }

    pub fn append_capability_invocation(
        &self,
        invocation: &CapabilityInvocation,
    ) -> EventStoreResult<()> {
        let event = KernelEvent::new(CAPABILITY_INVOCATION_RECORDED_EVENT, invocation)?;
        self.append(&event)
    }

    pub fn list_capability_invocations(&self) -> EventStoreResult<Vec<CapabilityInvocation>> {
        let events = self.list_by_type(CAPABILITY_INVOCATION_RECORDED_EVENT, 100)?;
        events
            .into_iter()
            .map(|event| {
                serde_json::from_str::<CapabilityInvocation>(&event.payload_json)
                    .map_err(Into::into)
            })
            .collect()
    }

    pub fn append_operations_briefing_run(
        &self,
        run: &OperationsBriefingRun,
    ) -> EventStoreResult<()> {
        let event = KernelEvent::new(OPERATIONS_BRIEFING_RUN_RECORDED_EVENT, run)?;
        self.append(&event)
    }

    pub fn list_operations_briefing_runs(&self) -> EventStoreResult<Vec<OperationsBriefingRun>> {
        let events = self.list_by_type(OPERATIONS_BRIEFING_RUN_RECORDED_EVENT, 100)?;
        events
            .into_iter()
            .map(|event| {
                serde_json::from_str::<OperationsBriefingRun>(&event.payload_json)
                    .map_err(Into::into)
            })
            .collect()
    }

    pub fn import_operations_briefing_runs(
        &self,
        runs: &[OperationsBriefingRun],
    ) -> EventStoreResult<WorkPackageOperationsBriefingImportSummary> {
        let mut existing_ids = self
            .list_operations_briefing_runs()?
            .into_iter()
            .map(|run| run.id)
            .collect::<std::collections::HashSet<_>>();
        let mut summary = WorkPackageOperationsBriefingImportSummary {
            imported: 0,
            skipped: 0,
        };

        for run in runs {
            if existing_ids.contains(&run.id) {
                summary.skipped += 1;
                continue;
            }

            let mut archived_run = run.clone();
            archived_run.archived_from_package = true;
            self.append_operations_briefing_run(&archived_run)?;
            existing_ids.insert(archived_run.id);
            summary.imported += 1;
        }

        Ok(summary)
    }

    fn list_by_type(&self, event_type: &str, limit: usize) -> EventStoreResult<Vec<KernelEvent>> {
        let limit = i64::try_from(limit).unwrap_or(i64::MAX);
        let mut statement = self.conn.prepare(
            r#"
            SELECT id, event_type, payload_json, created_at
            FROM kernel_events
            WHERE event_type = ?1
            ORDER BY created_at DESC
            LIMIT ?2
            "#,
        )?;
        let rows = statement
            .query_map(params![event_type, limit], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        let mut events = Vec::with_capacity(rows.len());
        for (id, event_type, payload_json, created_at) in rows {
            events.push(KernelEvent {
                id: Uuid::parse_str(&id)?,
                event_type,
                payload_json,
                created_at: DateTime::parse_from_rfc3339(&created_at)?.with_timezone(&Utc),
            });
        }

        Ok(events)
    }
}

fn push_unique_link(
    links_by_memory_id: &mut std::collections::HashMap<Uuid, Vec<Uuid>>,
    memory_id: Uuid,
    linked_memory_id: Uuid,
) {
    let links = links_by_memory_id.entry(memory_id).or_default();
    if !links.contains(&linked_memory_id) {
        links.push(linked_memory_id);
    }
}

fn push_unique_memory_body(bodies: &mut Vec<String>, body: &str) {
    let body = body.trim();
    if body.is_empty() || bodies.iter().any(|existing| existing == body) {
        return;
    }

    bodies.push(body.to_string());
}

fn normalize_memory_text(value: &str) -> String {
    value.trim().to_lowercase()
}

fn memory_candidate_conflicts_with_record(
    candidate: &MemoryCandidate,
    memory: &MemoryRecord,
) -> bool {
    if memory.source_id == Some(candidate.id) {
        return false;
    }

    let candidate_title = normalize_memory_text(&candidate.title);
    let memory_title = normalize_memory_text(&memory.title);
    if candidate_title == memory_title {
        return true;
    }

    if candidate.memory_type != memory.memory_type || candidate.scope != memory.scope {
        return false;
    }

    let candidate_body = normalize_memory_text(&candidate.body);
    let memory_body = normalize_memory_text(&memory.body);
    let long_enough_for_containment =
        candidate_body.chars().count() >= 18 && memory_body.chars().count() >= 18;

    candidate_body == memory_body
        || (long_enough_for_containment
            && (candidate_body.contains(&memory_body) || memory_body.contains(&candidate_body)))
}

fn capability_grant_state(
    request: &CapabilityAccessRequest,
    resolution: Option<&PermissionResolution>,
    effective_status: CapabilityAccessStatus,
    invocations: &[CapabilityInvocation],
) -> CapabilityGrantState {
    if effective_status != CapabilityAccessStatus::Approved {
        return CapabilityGrantState::NotGranted;
    }

    if capability_risk(request.capability) != RiskLevel::Critical {
        return CapabilityGrantState::Reusable;
    }

    let Some(resolution) = resolution else {
        return CapabilityGrantState::NotGranted;
    };

    let consumed = invocations.iter().any(|invocation| {
        if invocation.capability != request.capability
            || invocation.created_at < resolution.created_at
        {
            return false;
        }

        match invocation.approval_request_id {
            Some(approval_request_id) => approval_request_id == request.id,
            None => true,
        }
    });

    if consumed {
        CapabilityGrantState::OneShotConsumed
    } else {
        CapabilityGrantState::OneShotAvailable
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};
    use uuid::Uuid;

    use super::{EventStore, EventStoreError};
    use crate::kernel::capability::{CapabilityInvocation, CapabilityInvocationStatus};
    use crate::kernel::deepseek::{DeepSeekChatCacheStatus, DeepSeekChatTelemetry};
    use crate::kernel::models::{AccessMode, FoundationState};
    use crate::kernel::models::{
        KernelEvent, MemoryCandidate, MemoryCandidateSource, MemoryCandidateStatus,
        MemoryLifecycle, MemoryRecord, MemoryRecordSource, MemoryScope, MemorySensitivity,
        MemoryType, TaskRecord,
    };
    use crate::kernel::policy::{
        request_capability_access, CapabilityAccessStatus, CapabilityGrantState, CapabilityKind,
        PermissionAuditEntry,
    };
    use crate::kernel::work_package::export_work_package;
    use crate::kernel::workflow::WorkflowTemplatePackage;
    use crate::kernel::workflow::{
        OperationsBriefingAction, OperationsBriefingAnomaly, OperationsBriefingRun,
        OperationsBriefingRunStatus, OPERATIONS_BRIEFING_WORKFLOW_ID,
    };

    fn sample_operations_briefing_run() -> OperationsBriefingRun {
        OperationsBriefingRun {
            id: uuid::Uuid::new_v4(),
            workflow_id: OPERATIONS_BRIEFING_WORKFLOW_ID.to_string(),
            status: OperationsBriefingRunStatus::DraftReady,
            archived_from_package: false,
            evidence_folder_path: Some("fixtures/evidence".to_string()),
            evidence_invocation_id: Some(uuid::Uuid::new_v4()),
            title: "Operations Briefing Draft".to_string(),
            summary: "Draft ready from evidence folder manifest.".to_string(),
            anomalies: vec![OperationsBriefingAnomaly {
                area: "Evidence review".to_string(),
                signal: "Review accepted text files.".to_string(),
                evidence_ref: Some("fixtures/evidence".to_string()),
            }],
            action_plan: vec![OperationsBriefingAction {
                owner: "Operations owner".to_string(),
                action: "Confirm evidence set.".to_string(),
                due_hint: "Next briefing cycle".to_string(),
            }],
            warnings: Vec::new(),
            created_at: chrono::Utc::now(),
        }
    }

    #[test]
    fn appends_and_lists_recent_kernel_event() {
        let store = EventStore::open_memory().expect("memory store opens");
        let payload = serde_json::json!({
            "source": "foundation"
        });
        let event = KernelEvent::new("foundation.started", payload).expect("payload serializes");

        store.append(&event).expect("event appends");
        let events = store.list_recent(10).expect("recent events load");

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].id, event.id);
        assert_eq!(events[0].event_type, event.event_type);
        assert_eq!(events[0].payload_json, event.payload_json);
    }

    #[test]
    fn appends_and_lists_task_records() {
        let store = EventStore::open_memory().expect("memory store opens");
        let record = TaskRecord::new(
            "Review finance inbox".to_string(),
            "Collect evidence for the operations briefing.".to_string(),
        )
        .expect("record is valid");

        store
            .append_task_record(&record)
            .expect("task record appends");
        let records = store.list_task_records().expect("records load");

        assert_eq!(records, vec![record]);
    }

    #[test]
    fn imports_task_records_and_skips_existing_ids() {
        let store = EventStore::open_memory().expect("memory store opens");
        let existing = TaskRecord::new(
            "Review finance inbox".to_string(),
            "Collect evidence for the operations briefing.".to_string(),
        )
        .expect("record is valid");
        let incoming = TaskRecord::new(
            "Prepare weekly work package".to_string(),
            "Export task records for handoff.".to_string(),
        )
        .expect("record is valid");
        store
            .append_task_record(&existing)
            .expect("existing record appends");

        let summary = store
            .import_task_records(&[existing.clone(), incoming.clone()])
            .expect("records import");
        let records = store.list_task_records().expect("records load");

        assert_eq!(summary.imported, 1);
        assert_eq!(summary.skipped, 1);
        assert_eq!(records.len(), 2);
        assert!(records.contains(&existing));
        assert!(records.contains(&incoming));
    }

    #[test]
    fn captures_memory_from_task_record_once() {
        let store = EventStore::open_memory().expect("memory store opens");
        let task = TaskRecord::new(
            "Prepare executive summary".to_string(),
            "Remember the report needs source links and approval history.".to_string(),
        )
        .expect("task is valid");
        let memory = MemoryRecord::from_task_record(&task);

        store.append_memory_record(&memory).expect("memory appends");
        let duplicate = store
            .append_memory_record(&MemoryRecord::from_task_record(&task))
            .expect("duplicate memory is skipped");
        let memories = store.list_memory_records().expect("memories load");

        assert!(!duplicate);
        assert_eq!(memories, vec![memory]);
    }

    #[test]
    fn importing_task_records_captures_memory_for_new_records() {
        let store = EventStore::open_memory().expect("memory store opens");
        let existing = TaskRecord::new(
            "Review finance inbox".to_string(),
            "Collect evidence for the operations briefing.".to_string(),
        )
        .expect("record is valid");
        let incoming = TaskRecord::new(
            "Prepare weekly work package".to_string(),
            "Export task records and remember the handoff scope.".to_string(),
        )
        .expect("record is valid");
        store
            .append_task_record(&existing)
            .expect("existing record appends");
        store
            .append_memory_record(&MemoryRecord::from_task_record(&existing))
            .expect("existing memory appends");

        let summary = store
            .import_task_records(&[existing.clone(), incoming.clone()])
            .expect("records import");
        let memories = store.list_memory_records().expect("memories load");

        assert_eq!(summary.imported, 1);
        assert_eq!(summary.skipped, 1);
        assert_eq!(
            memories
                .iter()
                .filter(|memory| memory.source_id == Some(existing.id))
                .count(),
            1
        );
        assert!(memories
            .iter()
            .any(|memory| memory.source_id == Some(incoming.id)));
    }

    #[test]
    fn imported_memory_candidate_import_preview_counts_new_skipped_items_without_writing() {
        let store = EventStore::open_memory().expect("memory store opens");
        let existing = TaskRecord::new(
            "Existing task".to_string(),
            "Already present in the local event store.".to_string(),
        )
        .expect("record is valid");
        let incoming = TaskRecord::new(
            "Incoming handoff task".to_string(),
            "New task from a pasted work package.".to_string(),
        )
        .expect("record is valid");
        store
            .append_task_record(&existing)
            .expect("existing task appends");
        let existing_candidate = MemoryCandidate::new(
            "Existing memory candidate".to_string(),
            "This candidate is already in local review.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "Local reviewer already has this candidate.".to_string(),
        )
        .expect("candidate is valid");
        let incoming_candidate = MemoryCandidate::new(
            "Imported memory candidate".to_string(),
            "This candidate should be reviewed before becoming memory.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "Imported from a handoff package.".to_string(),
        )
        .expect("candidate is valid");
        store
            .append_memory_candidate(&existing_candidate)
            .expect("existing candidate appends");

        let package = export_work_package(
            FoundationState::default(),
            vec![existing.clone(), incoming],
            vec![existing_candidate, incoming_candidate],
            vec![sample_operations_briefing_run()],
        );
        let preview = store
            .preview_work_package_import(&package)
            .expect("preview loads");
        let records = store.list_task_records().expect("records load");

        assert_eq!(preview.task_records.total, 2);
        assert_eq!(preview.task_records.new, 1);
        assert_eq!(preview.task_records.skipped, 1);
        assert_eq!(preview.operations_briefing_runs.total, 1);
        assert!(preview.operations_briefing_runs.replay_supported);
        assert_eq!(preview.memory_candidates.total, 2);
        assert_eq!(preview.memory_candidates.new, 1);
        assert_eq!(preview.memory_candidates.skipped, 1);
        assert!(preview.memory_candidates.review_supported);
        assert_eq!(records, vec![existing]);
    }

    #[test]
    fn imported_memory_candidate_imports_new_candidates_as_pending_without_writing_memory() {
        let store = EventStore::open_memory().expect("memory store opens");
        let existing = MemoryCandidate::new(
            "Existing imported rule".to_string(),
            "This candidate is already present locally.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "Existing local review candidate.".to_string(),
        )
        .expect("candidate is valid");
        let incoming = MemoryCandidate::new_with_metadata(
            "Imported project context".to_string(),
            "Review this package context before saving it as local memory.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "Imported from a handoff package.".to_string(),
            MemoryType::ProjectContext,
            MemoryScope::Project,
            MemorySensitivity::Sensitive,
            MemoryLifecycle::Active,
        )
        .expect("candidate is valid");

        store
            .append_memory_candidate(&existing)
            .expect("existing candidate appends");

        let summary = store
            .import_memory_candidates(&[existing.clone(), incoming.clone()])
            .expect("candidates import");
        let records = store
            .list_memory_candidate_records()
            .expect("candidate records load");
        let memories = store.list_memory_records().expect("memories load");
        let imported = records
            .iter()
            .find(|record| record.candidate.id == incoming.id)
            .expect("incoming candidate imports");

        assert_eq!(summary.imported, 1);
        assert_eq!(summary.skipped, 1);
        assert_eq!(imported.effective_status, MemoryCandidateStatus::Pending);
        assert_eq!(imported.candidate.source, MemoryCandidateSource::Import);
        assert_eq!(imported.candidate.memory_type, MemoryType::ProjectContext);
        assert_eq!(imported.candidate.scope, MemoryScope::Project);
        assert_eq!(imported.candidate.sensitivity, MemorySensitivity::Sensitive);
        assert!(memories.is_empty());
    }

    #[test]
    fn workflow_template_package_import_preview_counts_new_and_skipped_templates() {
        let store = EventStore::open_memory().expect("memory store opens");
        let existing = WorkflowTemplatePackage::new(
            "operations.briefing.templates.v1".to_string(),
            "operations.briefing.v1".to_string(),
            "Operations Briefing Templates".to_string(),
            "Existing local template package.".to_string(),
            Vec::new(),
        )
        .expect("template package is valid");
        let incoming = WorkflowTemplatePackage::new(
            "operations.weekly-review.templates.v1".to_string(),
            "operations.weekly-review.v1".to_string(),
            "Weekly Review Templates".to_string(),
            "Incoming imported template package.".to_string(),
            Vec::new(),
        )
        .expect("template package is valid");
        store
            .append_workflow_template_package(&existing)
            .expect("existing template package appends");

        let mut package = export_work_package(
            FoundationState::default(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        package.workflow_templates = vec![existing, incoming];
        let preview = store
            .preview_work_package_import(&package)
            .expect("preview loads");

        assert_eq!(preview.workflow_templates.total, 2);
        assert_eq!(preview.workflow_templates.new, 1);
        assert_eq!(preview.workflow_templates.skipped, 1);
        assert!(preview.workflow_templates.import_supported);
    }

    #[test]
    fn workflow_template_package_import_adds_new_templates_once() {
        let store = EventStore::open_memory().expect("memory store opens");
        let existing = WorkflowTemplatePackage::new(
            "operations.briefing.templates.v1".to_string(),
            "operations.briefing.v1".to_string(),
            "Operations Briefing Templates".to_string(),
            "Existing local template package.".to_string(),
            Vec::new(),
        )
        .expect("template package is valid");
        let incoming = WorkflowTemplatePackage::new(
            "operations.weekly-review.templates.v1".to_string(),
            "operations.weekly-review.v1".to_string(),
            "Weekly Review Templates".to_string(),
            "Incoming imported template package.".to_string(),
            Vec::new(),
        )
        .expect("template package is valid");
        store
            .append_workflow_template_package(&existing)
            .expect("existing template package appends");

        let summary = store
            .import_workflow_template_packages(&[existing.clone(), incoming.clone()])
            .expect("template packages import");
        let templates = store
            .list_workflow_template_packages()
            .expect("template packages load");

        assert_eq!(summary.imported, 1);
        assert_eq!(summary.skipped, 1);
        assert_eq!(templates.len(), 2);
        assert!(templates.iter().any(|template| template.id == existing.id));
        assert!(templates.iter().any(|template| template.id == incoming.id));
    }

    #[test]
    fn searches_memory_records_by_title_and_body_case_insensitively() {
        let store = EventStore::open_memory().expect("memory store opens");
        let briefing = TaskRecord::new(
            "Prepare executive briefing".to_string(),
            "Include approval history and drive links.".to_string(),
        )
        .expect("record is valid");
        let browser = TaskRecord::new(
            "Review browser research".to_string(),
            "Capture competitor pricing notes.".to_string(),
        )
        .expect("record is valid");
        store
            .append_memory_record(&MemoryRecord::from_task_record(&briefing))
            .expect("briefing memory appends");
        store
            .append_memory_record(&MemoryRecord::from_task_record(&browser))
            .expect("browser memory appends");

        let title_matches = store
            .search_memory_records("BRIEF")
            .expect("title search works");
        let body_matches = store
            .search_memory_records("pricing")
            .expect("body search works");

        assert_eq!(title_matches.len(), 1);
        assert_eq!(title_matches[0].source_id, Some(briefing.id));
        assert_eq!(body_matches.len(), 1);
        assert_eq!(body_matches[0].source_id, Some(browser.id));
    }

    #[test]
    fn accepting_memory_candidate_writes_long_term_memory_once() {
        let store = EventStore::open_memory().expect("memory store opens");
        let candidate = MemoryCandidate::new(
            "Preferred report tone".to_string(),
            "Use concise operating language with clear owners and evidence.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "User proposed this as reusable guidance.".to_string(),
        )
        .expect("candidate is valid");

        store
            .append_memory_candidate(&candidate)
            .expect("candidate appends");
        let pending = store
            .list_memory_candidate_records()
            .expect("candidates load");

        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].effective_status, MemoryCandidateStatus::Pending);

        store
            .resolve_memory_candidate(candidate.id, true, "Looks reusable.".to_string())
            .expect("candidate resolves");
        let resolved = store
            .list_memory_candidate_records()
            .expect("candidates reload");
        let memories = store.list_memory_records().expect("memories load");

        assert_eq!(
            resolved[0].effective_status,
            MemoryCandidateStatus::Accepted
        );
        assert_eq!(memories.len(), 1);
        assert_eq!(memories[0].title, candidate.title);
        assert_eq!(memories[0].source, MemoryRecordSource::MemoryCandidate);
        assert_eq!(memories[0].source_id, Some(candidate.id));

        let duplicate = store
            .append_memory_record(&MemoryRecord::from_memory_candidate(&candidate))
            .expect("duplicate accepted memory is skipped");
        assert!(!duplicate);
    }

    #[test]
    fn linking_memory_candidate_accepts_candidate_and_keeps_related_memories() {
        let store = EventStore::open_memory().expect("memory store opens");
        let task = TaskRecord::new(
            "Preferred report tone".to_string(),
            "Use concise operating language.".to_string(),
        )
        .expect("task is valid");
        let existing_memory = MemoryRecord::from_task_record(&task);
        let candidate = MemoryCandidate::new(
            "Preferred report tone".to_string(),
            "Use concise operating language with owners and evidence.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "User wants to keep the related but richer instruction.".to_string(),
        )
        .expect("candidate is valid");

        store
            .append_memory_record(&existing_memory)
            .expect("memory appends");
        store
            .append_memory_candidate(&candidate)
            .expect("candidate appends");
        let resolution = store
            .link_memory_candidate_to_conflicts(
                candidate.id,
                vec![existing_memory.id],
                "Keep both memories and mark them related.".to_string(),
            )
            .expect("candidate links");
        let candidate_records = store
            .list_memory_candidate_records()
            .expect("candidate records load");
        let memories = store.list_memory_records().expect("memories load");
        let links = store.list_memory_record_links().expect("links load");
        let accepted_memory = memories
            .iter()
            .find(|memory| memory.source_id == Some(candidate.id))
            .expect("accepted memory is written");
        let original_memory = memories
            .iter()
            .find(|memory| memory.id == existing_memory.id)
            .expect("original memory is preserved");

        assert!(resolution.accepted);
        assert_eq!(resolution.candidate_id, candidate.id);
        assert_eq!(
            candidate_records[0].effective_status,
            MemoryCandidateStatus::Accepted
        );
        assert_eq!(memories.len(), 2);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].source_memory_id, accepted_memory.id);
        assert_eq!(links[0].target_memory_id, original_memory.id);
        assert_eq!(links[0].candidate_id, Some(candidate.id));
        assert_eq!(accepted_memory.linked_memory_ids, vec![original_memory.id]);
        assert_eq!(original_memory.linked_memory_ids, vec![accepted_memory.id]);
        assert_eq!(
            accepted_memory.linked_memories[0].title,
            original_memory.title
        );
        assert_eq!(
            original_memory.linked_memories[0].title,
            accepted_memory.title
        );
    }

    #[test]
    fn previewing_memory_candidate_merge_does_not_write_events() {
        let store = EventStore::open_memory().expect("memory store opens");
        let task = TaskRecord::new(
            "Preferred report tone".to_string(),
            "Use concise operating language.".to_string(),
        )
        .expect("task is valid");
        let existing_memory = MemoryRecord::from_task_record(&task);
        let candidate = MemoryCandidate::new_with_metadata(
            "Preferred report tone".to_string(),
            "Use concise operating language with owners and evidence.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "User wants a richer reusable instruction.".to_string(),
            MemoryType::WorkflowRule,
            MemoryScope::Project,
            MemorySensitivity::Sensitive,
            MemoryLifecycle::Active,
        )
        .expect("candidate is valid");

        store
            .append_memory_record(&existing_memory)
            .expect("memory appends");
        store
            .append_memory_candidate(&candidate)
            .expect("candidate appends");
        let preview = store
            .preview_memory_candidate_merge(candidate.id, vec![existing_memory.id])
            .expect("merge preview builds");
        let candidate_records = store
            .list_memory_candidate_records()
            .expect("candidate records load");
        let memories = store.list_memory_records().expect("memories load");
        let links = store.list_memory_record_links().expect("links load");

        assert_eq!(preview.candidate_id, candidate.id);
        assert_eq!(preview.source_memory_ids, vec![existing_memory.id]);
        assert_eq!(preview.title, candidate.title);
        assert!(preview.body.contains(&existing_memory.body));
        assert!(preview.body.contains(&candidate.body));
        assert_eq!(preview.memory_type, MemoryType::WorkflowRule);
        assert_eq!(preview.scope, MemoryScope::Project);
        assert_eq!(preview.sensitivity, MemorySensitivity::Sensitive);
        assert_eq!(preview.lifecycle, MemoryLifecycle::Active);
        assert_eq!(
            candidate_records[0].effective_status,
            MemoryCandidateStatus::Pending
        );
        assert_eq!(memories.len(), 1);
        assert!(links.is_empty());
    }

    #[test]
    fn previewing_memory_candidate_replace_does_not_write_events() {
        let store = EventStore::open_memory().expect("memory store opens");
        let task = TaskRecord::new(
            "Preferred report tone".to_string(),
            "Use concise operating language.".to_string(),
        )
        .expect("task is valid");
        let existing_memory = MemoryRecord::from_task_record(&task);
        let candidate = MemoryCandidate::new_with_metadata(
            "Preferred report tone".to_string(),
            "Use concise operating language with owners and evidence.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "User wants the richer instruction to supersede the old one.".to_string(),
            MemoryType::WorkflowRule,
            MemoryScope::Project,
            MemorySensitivity::Sensitive,
            MemoryLifecycle::Active,
        )
        .expect("candidate is valid");

        store
            .append_memory_record(&existing_memory)
            .expect("memory appends");
        store
            .append_memory_candidate(&candidate)
            .expect("candidate appends");
        let preview = store
            .preview_memory_candidate_replace(candidate.id, vec![existing_memory.id])
            .expect("replace preview builds");
        let candidate_records = store
            .list_memory_candidate_records()
            .expect("candidate records load");
        let memories = store.list_memory_records().expect("memories load");
        let deletions = store
            .list_memory_record_deletions()
            .expect("deletions load");
        let links = store.list_memory_record_links().expect("links load");

        assert_eq!(preview.candidate_id, candidate.id);
        assert_eq!(preview.target_memory_ids, vec![existing_memory.id]);
        assert_eq!(preview.replacement_title, candidate.title);
        assert_eq!(preview.replacement_body, candidate.body);
        assert_eq!(preview.target_memories.len(), 1);
        assert_eq!(preview.target_memories[0].id, existing_memory.id);
        assert_eq!(preview.memory_type, MemoryType::WorkflowRule);
        assert_eq!(preview.scope, MemoryScope::Project);
        assert_eq!(preview.sensitivity, MemorySensitivity::Sensitive);
        assert_eq!(preview.lifecycle, MemoryLifecycle::Active);
        assert_eq!(
            candidate_records[0].effective_status,
            MemoryCandidateStatus::Pending
        );
        assert_eq!(memories.len(), 1);
        assert!(deletions.is_empty());
        assert!(links.is_empty());
    }

    #[test]
    fn merging_memory_candidate_accepts_merged_memory_and_hides_sources() {
        let store = EventStore::open_memory().expect("memory store opens");
        let task = TaskRecord::new(
            "Preferred report tone".to_string(),
            "Use concise operating language.".to_string(),
        )
        .expect("task is valid");
        let existing_memory = MemoryRecord::from_task_record(&task);
        let candidate = MemoryCandidate::new_with_metadata(
            "Preferred report tone".to_string(),
            "Use concise operating language with owners and evidence.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "User wants a richer reusable instruction.".to_string(),
            MemoryType::WorkflowRule,
            MemoryScope::Project,
            MemorySensitivity::Sensitive,
            MemoryLifecycle::Active,
        )
        .expect("candidate is valid");

        store
            .append_memory_record(&existing_memory)
            .expect("memory appends");
        store
            .append_memory_candidate(&candidate)
            .expect("candidate appends");
        let resolution = store
            .merge_memory_candidate_with_conflicts(
                candidate.id,
                vec![existing_memory.id],
                "Merge and accept richer memory.".to_string(),
            )
            .expect("candidate merges");
        let candidate_records = store
            .list_memory_candidate_records()
            .expect("candidate records load");
        let memories = store.list_memory_records().expect("memories load");
        let deletions = store
            .list_memory_record_deletions()
            .expect("deletions load");
        let links = store.list_memory_record_links().expect("links load");

        assert!(resolution.accepted);
        assert_eq!(resolution.candidate_id, candidate.id);
        assert_eq!(
            candidate_records[0].effective_status,
            MemoryCandidateStatus::Accepted
        );
        assert_eq!(memories.len(), 1);
        assert_eq!(memories[0].source, MemoryRecordSource::MemoryCandidate);
        assert_eq!(memories[0].source_id, Some(candidate.id));
        assert!(memories[0].body.contains(&existing_memory.body));
        assert!(memories[0].body.contains(&candidate.body));
        assert_eq!(memories[0].memory_type, MemoryType::WorkflowRule);
        assert_eq!(memories[0].scope, MemoryScope::Project);
        assert_eq!(memories[0].sensitivity, MemorySensitivity::Sensitive);
        assert_eq!(deletions.len(), 1);
        assert_eq!(deletions[0].memory_id, existing_memory.id);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target_memory_id, existing_memory.id);
        assert_eq!(links[0].candidate_id, Some(candidate.id));
    }

    #[test]
    fn replacing_memory_candidate_accepts_replacement_and_tombstones_targets() {
        let store = EventStore::open_memory().expect("memory store opens");
        let task = TaskRecord::new(
            "Preferred report tone".to_string(),
            "Use concise operating language.".to_string(),
        )
        .expect("task is valid");
        let existing_memory = MemoryRecord::from_task_record(&task);
        let candidate = MemoryCandidate::new_with_metadata(
            "Preferred report tone".to_string(),
            "Use concise operating language with owners and evidence.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "User wants the richer instruction to supersede the old one.".to_string(),
            MemoryType::WorkflowRule,
            MemoryScope::Project,
            MemorySensitivity::Sensitive,
            MemoryLifecycle::Active,
        )
        .expect("candidate is valid");

        store
            .append_memory_record(&existing_memory)
            .expect("memory appends");
        store
            .append_memory_candidate(&candidate)
            .expect("candidate appends");
        let resolution = store
            .replace_memory_candidate_conflicts(
                candidate.id,
                vec![existing_memory.id],
                "Replace with accepted candidate.".to_string(),
            )
            .expect("candidate replaces");
        let candidate_records = store
            .list_memory_candidate_records()
            .expect("candidate records load");
        let memories = store.list_memory_records().expect("memories load");
        let deletions = store
            .list_memory_record_deletions()
            .expect("deletions load");
        let links = store.list_memory_record_links().expect("links load");

        assert!(resolution.accepted);
        assert_eq!(resolution.candidate_id, candidate.id);
        assert_eq!(
            candidate_records[0].effective_status,
            MemoryCandidateStatus::Accepted
        );
        assert_eq!(memories.len(), 1);
        assert_eq!(memories[0].source, MemoryRecordSource::MemoryCandidate);
        assert_eq!(memories[0].source_id, Some(candidate.id));
        assert_eq!(memories[0].body, candidate.body);
        assert!(!memories[0].body.contains(&existing_memory.body));
        assert_eq!(deletions.len(), 1);
        assert_eq!(deletions[0].memory_id, existing_memory.id);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target_memory_id, existing_memory.id);
        assert_eq!(links[0].candidate_id, Some(candidate.id));
    }

    #[test]
    fn rejecting_memory_candidate_does_not_write_memory() {
        let store = EventStore::open_memory().expect("memory store opens");
        let candidate = MemoryCandidate::new(
            "Temporary report instruction".to_string(),
            "Only applies to today's draft.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "User proposed this as reusable guidance.".to_string(),
        )
        .expect("candidate is valid");

        store
            .append_memory_candidate(&candidate)
            .expect("candidate appends");
        store
            .resolve_memory_candidate(candidate.id, false, "Too temporary.".to_string())
            .expect("candidate rejects");
        let resolved = store
            .list_memory_candidate_records()
            .expect("candidates reload");
        let memories = store.list_memory_records().expect("memories load");

        assert_eq!(
            resolved[0].effective_status,
            MemoryCandidateStatus::Rejected
        );
        assert!(memories.is_empty());
    }

    #[test]
    fn memory_candidate_records_surface_conflicting_memory_ids() {
        let store = EventStore::open_memory().expect("memory store opens");
        let task = TaskRecord::new(
            "Preferred report tone".to_string(),
            "Use concise operating language.".to_string(),
        )
        .expect("task is valid");
        let memory = MemoryRecord::from_task_record(&task);
        let candidate = MemoryCandidate::new(
            "Preferred report tone".to_string(),
            "Use concise operating language with owners and evidence.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "User proposed this as reusable guidance.".to_string(),
        )
        .expect("candidate is valid");

        store.append_memory_record(&memory).expect("memory appends");
        store
            .append_memory_candidate(&candidate)
            .expect("candidate appends");
        let records = store
            .list_memory_candidate_records()
            .expect("candidates load");

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].conflicting_memory_ids, vec![memory.id]);
        assert_eq!(records[0].conflicting_memories.len(), 1);
        assert_eq!(records[0].conflicting_memories[0].id, memory.id);
        assert_eq!(
            records[0].conflicting_memories[0].title,
            "Preferred report tone"
        );
        assert_eq!(
            records[0].conflicting_memories[0].body,
            "Use concise operating language."
        );
    }

    #[test]
    fn memory_candidate_conflicts_ignore_deleted_memories() {
        let store = EventStore::open_memory().expect("memory store opens");
        let task = TaskRecord::new(
            "Retired memory".to_string(),
            "This memory is no longer active.".to_string(),
        )
        .expect("task is valid");
        let memory = MemoryRecord::from_task_record(&task);
        let candidate = MemoryCandidate::new(
            "Retired memory".to_string(),
            "A fresh candidate with the same title.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "User proposed this as reusable guidance.".to_string(),
        )
        .expect("candidate is valid");

        store.append_memory_record(&memory).expect("memory appends");
        store
            .delete_memory_record(memory.id, "No longer useful.".to_string())
            .expect("memory deletes");
        store
            .append_memory_candidate(&candidate)
            .expect("candidate appends");
        let records = store
            .list_memory_candidate_records()
            .expect("candidates load");

        assert_eq!(records.len(), 1);
        assert!(records[0].conflicting_memory_ids.is_empty());
    }

    #[test]
    fn deleting_memory_record_hides_it_from_list_and_search() {
        let store = EventStore::open_memory().expect("memory store opens");
        let task = TaskRecord::new(
            "Memory cleanup".to_string(),
            "Keep accepted memory reviewable.".to_string(),
        )
        .expect("task is valid");
        let memory = MemoryRecord::from_task_record(&task);

        store.append_memory_record(&memory).expect("memory appends");
        let deletion = store
            .delete_memory_record(memory.id, "No longer useful.".to_string())
            .expect("memory deletes");

        assert_eq!(deletion.memory_id, memory.id);
        assert_eq!(deletion.note, "No longer useful.");
        assert!(store
            .list_memory_records()
            .expect("memories load")
            .is_empty());
        assert!(store
            .search_memory_records("cleanup")
            .expect("memories search")
            .is_empty());
    }

    #[test]
    fn deleting_missing_memory_record_returns_not_found() {
        let store = EventStore::open_memory().expect("memory store opens");
        let error = store
            .delete_memory_record(Uuid::new_v4(), "Missing.".to_string())
            .expect_err("missing memory cannot be deleted");

        assert!(matches!(error, EventStoreError::NotFound(_)));
    }

    #[test]
    fn updating_memory_record_replaces_visible_version_for_list_and_search() {
        let store = EventStore::open_memory().expect("memory store opens");
        let task = TaskRecord::new(
            "Old memory title".to_string(),
            "Old body should stop matching searches.".to_string(),
        )
        .expect("task is valid");
        let memory = MemoryRecord::from_task_record(&task);

        store.append_memory_record(&memory).expect("memory appends");
        let update = store
            .update_memory_record(
                memory.id,
                "Updated memory title".to_string(),
                "New body should be searchable.".to_string(),
                MemoryType::WorkflowRule,
                MemoryScope::Project,
                MemorySensitivity::Sensitive,
                MemoryLifecycle::Archived,
                None,
                "User corrected the accepted memory.".to_string(),
            )
            .expect("memory updates");

        let memories = store.list_memory_records().expect("memories load");
        let old_matches = store
            .search_memory_records("old body")
            .expect("old body search works");
        let new_matches = store
            .search_memory_records("new body")
            .expect("new body search works");

        assert_eq!(update.memory_id, memory.id);
        assert_eq!(update.note, "User corrected the accepted memory.");
        assert_eq!(memories.len(), 1);
        assert_eq!(memories[0].id, memory.id);
        assert_eq!(memories[0].title, "Updated memory title");
        assert_eq!(memories[0].body, "New body should be searchable.");
        assert_eq!(memories[0].memory_type, MemoryType::WorkflowRule);
        assert_eq!(memories[0].scope, MemoryScope::Project);
        assert_eq!(memories[0].sensitivity, MemorySensitivity::Sensitive);
        assert_eq!(memories[0].lifecycle, MemoryLifecycle::Archived);
        assert_eq!(memories[0].source, MemoryRecordSource::TaskRecord);
        assert_eq!(memories[0].source_id, Some(task.id));
        assert_eq!(memories[0].created_at, memory.created_at);
        assert!(memories[0].updated_at >= memory.updated_at);
        assert!(old_matches.is_empty());
        assert_eq!(new_matches.len(), 1);
        assert_eq!(new_matches[0].id, memory.id);
    }

    #[test]
    fn updating_deleted_memory_record_returns_not_found() {
        let store = EventStore::open_memory().expect("memory store opens");
        let task = TaskRecord::new(
            "Deleted memory".to_string(),
            "This memory should not be editable.".to_string(),
        )
        .expect("task is valid");
        let memory = MemoryRecord::from_task_record(&task);

        store.append_memory_record(&memory).expect("memory appends");
        store
            .delete_memory_record(memory.id, "Remove before editing.".to_string())
            .expect("memory deletes");
        let error = store
            .update_memory_record(
                memory.id,
                "Edited deleted memory".to_string(),
                "This should not be written.".to_string(),
                MemoryType::Preference,
                MemoryScope::Workspace,
                MemorySensitivity::Normal,
                MemoryLifecycle::Active,
                None,
                "Attempted edit after deletion.".to_string(),
            )
            .expect_err("deleted memory cannot be updated");

        assert!(matches!(error, EventStoreError::NotFound(_)));
    }

    #[test]
    fn expired_memory_records_are_hidden_from_list_and_search() {
        let store = EventStore::open_memory().expect("memory store opens");
        let now = Utc::now();
        let mut memory = MemoryRecord::from_task_record(
            &TaskRecord::new(
                "Expired operating note".to_string(),
                "This instruction should no longer guide the agent.".to_string(),
            )
            .expect("task is valid"),
        );
        memory.lifecycle = MemoryLifecycle::Expires;
        memory.expires_at = Some(now - Duration::days(1));

        store.append_memory_record(&memory).expect("memory appends");

        assert!(store
            .list_memory_records_at(now)
            .expect("memories load")
            .is_empty());
        assert!(store
            .search_memory_records_at("operating", now)
            .expect("memories search")
            .is_empty());
    }

    #[test]
    fn future_expiring_memory_candidate_preserves_expiration_when_accepted() {
        let store = EventStore::open_memory().expect("memory store opens");
        let now = Utc::now();
        let expires_at = now + Duration::days(30);
        let candidate = MemoryCandidate::new_with_metadata_and_expiration(
            "Quarterly briefing rule".to_string(),
            "Use this guidance until the current quarterly cycle closes.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "User proposed this as time-bound guidance.".to_string(),
            MemoryType::WorkflowRule,
            MemoryScope::Project,
            MemorySensitivity::Normal,
            MemoryLifecycle::Expires,
            Some(expires_at),
        )
        .expect("candidate is valid");

        store
            .append_memory_candidate(&candidate)
            .expect("candidate appends");
        store
            .resolve_memory_candidate(candidate.id, true, "Accept timed rule.".to_string())
            .expect("candidate resolves");
        let memories = store.list_memory_records_at(now).expect("memories load");

        assert_eq!(memories.len(), 1);
        assert_eq!(memories[0].title, candidate.title);
        assert_eq!(memories[0].lifecycle, MemoryLifecycle::Expires);
        assert_eq!(memories[0].expires_at, Some(expires_at));
    }

    #[test]
    fn memory_metadata_accepting_candidate_preserves_review_tags() {
        let store = EventStore::open_memory().expect("memory store opens");
        let candidate = MemoryCandidate::new_with_metadata(
            "Evidence routing rule".to_string(),
            "Keep source scans as the highest authority when restoring text.".to_string(),
            MemoryCandidateSource::Manual,
            None,
            "User confirmed this as reusable workflow guidance.".to_string(),
            MemoryType::WorkflowRule,
            MemoryScope::Project,
            MemorySensitivity::Sensitive,
            MemoryLifecycle::Active,
        )
        .expect("candidate is valid");

        store
            .append_memory_candidate(&candidate)
            .expect("candidate appends");
        store
            .resolve_memory_candidate(candidate.id, true, "Promote rule.".to_string())
            .expect("candidate resolves");
        let memories = store.list_memory_records().expect("memories load");

        assert_eq!(memories.len(), 1);
        assert_eq!(memories[0].source_id, Some(candidate.id));
        assert_eq!(memories[0].memory_type, MemoryType::WorkflowRule);
        assert_eq!(memories[0].scope, MemoryScope::Project);
        assert_eq!(memories[0].sensitivity, MemorySensitivity::Sensitive);
        assert_eq!(memories[0].lifecycle, MemoryLifecycle::Active);
    }

    #[test]
    fn appends_and_lists_permission_audit_entries() {
        let store = EventStore::open_memory().expect("memory store opens");
        let entry =
            PermissionAuditEntry::evaluate(AccessMode::AskOnRisk, CapabilityKind::BrowserBrowse);

        store
            .append_permission_audit_entry(&entry)
            .expect("permission audit appends");
        let entries = store
            .list_permission_audit_entries()
            .expect("permission audits load");

        assert_eq!(entries, vec![entry]);
    }

    #[test]
    fn appends_and_lists_deepseek_chat_telemetry() {
        let store = EventStore::open_memory().expect("memory store opens");
        let telemetry = DeepSeekChatTelemetry {
            id: Uuid::new_v4(),
            request_hash: "abc123".to_string(),
            model: "deepseek-v4-pro".to_string(),
            cache_status: DeepSeekChatCacheStatus::Miss,
            elapsed_ms: 42,
            prompt_tokens: Some(100),
            completion_tokens: Some(20),
            total_tokens: Some(120),
            estimated_cost_micro_usd: None,
            created_at: chrono::Utc::now(),
        };

        store
            .append_deepseek_chat_telemetry(&telemetry)
            .expect("telemetry appends");
        let entries = store
            .list_deepseek_chat_telemetry()
            .expect("telemetry loads");

        assert_eq!(entries, vec![telemetry]);
    }

    #[test]
    fn resolves_pending_capability_access_request() {
        let store = EventStore::open_memory().expect("memory store opens");
        let request = request_capability_access(AccessMode::FullAccess, CapabilityKind::EmailSend)
            .expect("email send request builds");

        store
            .append_capability_access_request(&request)
            .expect("request appends");
        let pending = store
            .list_pending_capability_access_records()
            .expect("pending requests load");
        assert_eq!(pending.len(), 1);
        assert_eq!(
            pending[0].effective_status,
            CapabilityAccessStatus::PendingApproval
        );

        let resolution = store
            .resolve_capability_access_request(
                request.id,
                true,
                "Approved after user reviewed the outbound message.".to_string(),
            )
            .expect("request resolves");
        let pending_after_resolution = store
            .list_pending_capability_access_records()
            .expect("pending requests reload");
        let records = store
            .list_capability_access_records()
            .expect("access records load");

        assert!(resolution.approved);
        assert!(pending_after_resolution.is_empty());
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].request.id, request.id);
        assert_eq!(
            records[0].effective_status,
            CapabilityAccessStatus::Approved
        );
        assert_eq!(
            records[0]
                .resolution
                .as_ref()
                .expect("resolution exists")
                .note,
            "Approved after user reviewed the outbound message."
        );
    }

    #[test]
    fn auto_approved_capability_access_request_is_not_pending() {
        let store = EventStore::open_memory().expect("memory store opens");
        let request = request_capability_access(AccessMode::AskOnRisk, CapabilityKind::DriveRead)
            .expect("drive read request builds");

        store
            .append_capability_access_request(&request)
            .expect("request appends");
        let pending = store
            .list_pending_capability_access_records()
            .expect("pending requests load");
        let records = store
            .list_capability_access_records()
            .expect("access records load");

        assert!(pending.is_empty());
        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].effective_status,
            CapabilityAccessStatus::AutoApproved
        );
    }

    #[test]
    fn appends_and_lists_capability_invocations() {
        let store = EventStore::open_memory().expect("memory store opens");
        let invocation = CapabilityInvocation {
            id: uuid::Uuid::new_v4(),
            capability: CapabilityKind::BrowserBrowse,
            status: CapabilityInvocationStatus::Succeeded,
            policy_decision: crate::kernel::policy::PolicyDecision::Allow,
            approval_request_id: None,
            requested_resource: Some("https://example.com".to_string()),
            evidence_ref: Some("https://example.com/final".to_string()),
            requested_url: Some("https://example.com".to_string()),
            evidence_url: Some("https://example.com/final".to_string()),
            title: Some("Example Domain".to_string()),
            excerpt: Some("Example evidence text".to_string()),
            warnings: Vec::new(),
            elapsed_ms: 24,
            created_at: chrono::Utc::now(),
        };

        store
            .append_capability_invocation(&invocation)
            .expect("invocation appends");
        let invocations = store
            .list_capability_invocations()
            .expect("invocations load");

        assert_eq!(invocations, vec![invocation]);
    }

    #[test]
    fn appends_and_lists_operations_briefing_runs() {
        let store = EventStore::open_memory().expect("memory store opens");
        let run = OperationsBriefingRun {
            id: uuid::Uuid::new_v4(),
            workflow_id: OPERATIONS_BRIEFING_WORKFLOW_ID.to_string(),
            status: OperationsBriefingRunStatus::DraftReady,
            archived_from_package: false,
            evidence_folder_path: Some("fixtures/evidence".to_string()),
            evidence_invocation_id: Some(uuid::Uuid::new_v4()),
            title: "Operations Briefing Draft".to_string(),
            summary: "Draft ready from evidence folder manifest.".to_string(),
            anomalies: vec![OperationsBriefingAnomaly {
                area: "Evidence review".to_string(),
                signal: "Review accepted text files.".to_string(),
                evidence_ref: Some("fixtures/evidence".to_string()),
            }],
            action_plan: vec![OperationsBriefingAction {
                owner: "Operations owner".to_string(),
                action: "Confirm evidence set.".to_string(),
                due_hint: "Next briefing cycle".to_string(),
            }],
            warnings: Vec::new(),
            created_at: chrono::Utc::now(),
        };

        store
            .append_operations_briefing_run(&run)
            .expect("operations briefing run appends");
        let runs = store
            .list_operations_briefing_runs()
            .expect("operations briefing runs load");

        assert_eq!(runs, vec![run]);
    }

    #[test]
    fn archive_replay_imports_new_runs_as_archived_and_skips_existing_ids() {
        let store = EventStore::open_memory().expect("memory store opens");
        let existing = sample_operations_briefing_run();
        let incoming = sample_operations_briefing_run();
        store
            .append_operations_briefing_run(&existing)
            .expect("existing run appends");

        let summary = store
            .import_operations_briefing_runs(&[existing.clone(), incoming.clone()])
            .expect("runs import");
        let runs = store
            .list_operations_briefing_runs()
            .expect("operations briefing runs load");
        let imported = runs
            .iter()
            .find(|run| run.id == incoming.id)
            .expect("incoming run is imported");
        let existing_after_import = runs
            .iter()
            .find(|run| run.id == existing.id)
            .expect("existing run is still present");

        assert_eq!(summary.imported, 1);
        assert_eq!(summary.skipped, 1);
        assert!(imported.archived_from_package);
        assert!(!existing_after_import.archived_from_package);
    }

    #[test]
    fn reusable_capability_grant_requires_explicit_user_approval() {
        let store = EventStore::open_memory().expect("memory store opens");
        let auto_request =
            request_capability_access(AccessMode::LimitedAuto, CapabilityKind::BrowserBrowse)
                .expect("auto-approved browser request builds");
        store
            .append_capability_access_request(&auto_request)
            .expect("auto-approved request appends");

        assert!(!store
            .has_user_approved_capability(CapabilityKind::BrowserBrowse)
            .expect("grant check works"));

        let pending_request =
            request_capability_access(AccessMode::AskOnRisk, CapabilityKind::BrowserBrowse)
                .expect("pending browser request builds");
        store
            .append_capability_access_request(&pending_request)
            .expect("pending request appends");
        store
            .resolve_capability_access_request(
                pending_request.id,
                true,
                "User approved browser browsing.".to_string(),
            )
            .expect("pending request resolves");

        assert!(store
            .has_user_approved_capability(CapabilityKind::BrowserBrowse)
            .expect("grant check works"));
        let records = store
            .list_capability_access_records()
            .expect("records load");
        let approved_record = records
            .iter()
            .find(|record| record.request.id == pending_request.id)
            .expect("approved browser record exists");
        assert_eq!(approved_record.grant_state, CapabilityGrantState::Reusable);
    }

    #[test]
    fn critical_capability_approval_is_consumed_after_next_invocation() {
        let store = EventStore::open_memory().expect("memory store opens");
        let request = request_capability_access(AccessMode::FullAccess, CapabilityKind::EmailSend)
            .expect("critical request builds");
        store
            .append_capability_access_request(&request)
            .expect("request appends");
        let resolution = store
            .resolve_capability_access_request(
                request.id,
                true,
                "Approved one outbound email.".to_string(),
            )
            .expect("request resolves");

        assert!(store
            .has_user_approved_capability(CapabilityKind::EmailSend)
            .expect("grant check works"));
        let records_before_invocation = store
            .list_capability_access_records()
            .expect("records load before invocation");
        assert_eq!(
            records_before_invocation[0].grant_state,
            CapabilityGrantState::OneShotAvailable
        );

        store
            .append_capability_invocation(&CapabilityInvocation {
                id: uuid::Uuid::new_v4(),
                capability: CapabilityKind::EmailSend,
                status: CapabilityInvocationStatus::Failed,
                policy_decision: crate::kernel::policy::PolicyDecision::Ask,
                approval_request_id: None,
                requested_resource: Some("ops@example.com".to_string()),
                evidence_ref: Some("ops@example.com".to_string()),
                requested_url: None,
                evidence_url: None,
                title: Some("Email send blocked: Weekly brief".to_string()),
                excerpt: Some("Approved email send attempt.".to_string()),
                warnings: vec!["email send execution is not enabled".to_string()],
                elapsed_ms: 1,
                created_at: resolution.created_at + chrono::Duration::milliseconds(1),
            })
            .expect("invocation appends");

        assert!(!store
            .has_user_approved_capability(CapabilityKind::EmailSend)
            .expect("grant check works"));
        let records_after_invocation = store
            .list_capability_access_records()
            .expect("records load after invocation");
        assert_eq!(
            records_after_invocation[0].grant_state,
            CapabilityGrantState::OneShotConsumed
        );
    }

    #[test]
    fn critical_capability_consumption_prefers_explicit_approval_request_id() {
        let store = EventStore::open_memory().expect("memory store opens");
        let first_request =
            request_capability_access(AccessMode::FullAccess, CapabilityKind::EmailSend)
                .expect("first critical request builds");
        store
            .append_capability_access_request(&first_request)
            .expect("first request appends");
        store
            .resolve_capability_access_request(
                first_request.id,
                true,
                "Approved first outbound email.".to_string(),
            )
            .expect("first request resolves");

        let second_request =
            request_capability_access(AccessMode::FullAccess, CapabilityKind::EmailSend)
                .expect("second critical request builds");
        store
            .append_capability_access_request(&second_request)
            .expect("second request appends");
        let second_resolution = store
            .resolve_capability_access_request(
                second_request.id,
                true,
                "Approved second outbound email.".to_string(),
            )
            .expect("second request resolves");

        store
            .append_capability_invocation(&CapabilityInvocation {
                id: uuid::Uuid::new_v4(),
                capability: CapabilityKind::EmailSend,
                status: CapabilityInvocationStatus::Failed,
                policy_decision: crate::kernel::policy::PolicyDecision::Ask,
                approval_request_id: Some(first_request.id),
                requested_resource: Some("ops@example.com".to_string()),
                evidence_ref: Some("ops@example.com".to_string()),
                requested_url: None,
                evidence_url: None,
                title: Some("Email send blocked: First brief".to_string()),
                excerpt: Some("First approved email send attempt.".to_string()),
                warnings: vec!["email send execution is not enabled".to_string()],
                elapsed_ms: 1,
                created_at: second_resolution.created_at + chrono::Duration::milliseconds(1),
            })
            .expect("linked invocation appends");

        let records = store
            .list_capability_access_records()
            .expect("records load");
        let first_record = records
            .iter()
            .find(|record| record.request.id == first_request.id)
            .expect("first record exists");
        let second_record = records
            .iter()
            .find(|record| record.request.id == second_request.id)
            .expect("second record exists");

        assert_eq!(
            first_record.grant_state,
            CapabilityGrantState::OneShotConsumed
        );
        assert_eq!(
            second_record.grant_state,
            CapabilityGrantState::OneShotAvailable
        );
    }
}
