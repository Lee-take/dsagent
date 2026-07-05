use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AgentLoopMode {
    DirectAnswer,
    EvidenceGathering,
    PermissionedAction,
    WorkflowRun,
    CodingRepair,
    Review,
    Verification,
    Resume,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AgentLoopModeDescriptor {
    pub mode: AgentLoopMode,
    pub allowed_tools: &'static [&'static str],
    pub validators: &'static [&'static str],
    pub stop_conditions: &'static [&'static str],
    pub confirmation_rule: &'static str,
}

impl AgentLoopMode {
    pub fn as_str(self) -> &'static str {
        match self {
            AgentLoopMode::DirectAnswer => "direct_answer",
            AgentLoopMode::EvidenceGathering => "evidence_gathering",
            AgentLoopMode::PermissionedAction => "permissioned_action",
            AgentLoopMode::WorkflowRun => "workflow_run",
            AgentLoopMode::CodingRepair => "coding_repair",
            AgentLoopMode::Review => "review",
            AgentLoopMode::Verification => "verification",
            AgentLoopMode::Resume => "resume",
        }
    }
}

pub fn agent_loop_mode_descriptor(mode: AgentLoopMode) -> AgentLoopModeDescriptor {
    match mode {
        AgentLoopMode::DirectAnswer => AgentLoopModeDescriptor {
            mode,
            allowed_tools: &[],
            validators: &["schema_normalized"],
            stop_conditions: &["answer_ready", "missing_prerequisite"],
            confirmation_rule: "no local tool confirmation needed",
        },
        AgentLoopMode::EvidenceGathering => AgentLoopModeDescriptor {
            mode,
            allowed_tools: &[
                "browser_browse",
                "computer_screenshot",
                "file_read",
                "network_search",
                "terminal_read",
            ],
            validators: &[
                "schema_normalized",
                "capability_policy_checked",
                "evidence_reference_recorded",
            ],
            stop_conditions: &[
                "evidence_observed",
                "blocked_or_failed",
                "user_confirmation_required",
            ],
            confirmation_rule: "follow capability policy before tool dispatch",
        },
        AgentLoopMode::PermissionedAction => AgentLoopModeDescriptor {
            mode,
            allowed_tools: &[
                "browser_submit",
                "computer_control",
                "create_report",
                "email_draft",
                "email_send",
                "file_write",
                "terminal_write",
            ],
            validators: &[
                "schema_normalized",
                "capability_policy_checked",
                "approval_state_checked",
            ],
            stop_conditions: &[
                "action_completed",
                "blocked_or_failed",
                "user_confirmation_required",
            ],
            confirmation_rule: "explicit user approval when capability policy asks",
        },
        AgentLoopMode::WorkflowRun => AgentLoopModeDescriptor {
            mode,
            allowed_tools: &["operations_briefing"],
            validators: &[
                "schema_normalized",
                "capability_policy_checked",
                "workflow_run_recorded",
            ],
            stop_conditions: &[
                "workflow_draft_ready",
                "blocked_or_failed",
                "user_confirmation_required",
            ],
            confirmation_rule: "workflow policy and capability policy both apply",
        },
        AgentLoopMode::CodingRepair => AgentLoopModeDescriptor {
            mode,
            allowed_tools: &["file_read", "file_write", "terminal_read"],
            validators: &["test_failure_reproduced", "focused_fix_verified"],
            stop_conditions: &[
                "tests_passed",
                "repair_budget_exhausted",
                "blocked_or_failed",
            ],
            confirmation_rule: "ask before destructive or broad filesystem changes",
        },
        AgentLoopMode::Review => AgentLoopModeDescriptor {
            mode,
            allowed_tools: &["file_read", "memory_candidate"],
            validators: &["review_item_recorded", "no_silent_memory_write"],
            stop_conditions: &["review_item_queued", "blocked_or_failed"],
            confirmation_rule: "review writes wait for user acceptance",
        },
        AgentLoopMode::Verification => AgentLoopModeDescriptor {
            mode,
            allowed_tools: &["file_read", "terminal_read"],
            validators: &["verification_command_reviewed", "result_recorded"],
            stop_conditions: &["verification_passed", "verification_failed"],
            confirmation_rule: "use read-only checks unless user authorizes more",
        },
        AgentLoopMode::Resume => AgentLoopModeDescriptor {
            mode,
            allowed_tools: &["file_read", "work_package_export"],
            validators: &["resume_state_loaded", "next_action_identified"],
            stop_conditions: &["resume_ready", "missing_prerequisite"],
            confirmation_rule: "ask when resuming would execute a pending risky action",
        },
    }
}

pub fn classify_agent_action_loop_mode(
    action_type: &str,
    execution_state: &str,
    requires_confirmation: bool,
    has_workflow_run: bool,
) -> AgentLoopMode {
    let action_type = action_type.trim();
    let execution_state = execution_state.trim();
    if has_workflow_run || action_type == "operations_briefing" {
        return AgentLoopMode::WorkflowRun;
    }
    if requires_confirmation || execution_state == "needs_confirmation" {
        return AgentLoopMode::PermissionedAction;
    }
    if matches!(action_type, "memory_candidate") {
        return AgentLoopMode::Review;
    }
    if agent_action_type_is_evidence_gathering(action_type) {
        return AgentLoopMode::EvidenceGathering;
    }
    if agent_action_type_is_permissioned_action(action_type) {
        return AgentLoopMode::PermissionedAction;
    }
    AgentLoopMode::DirectAnswer
}

fn agent_action_type_is_evidence_gathering(action_type: &str) -> bool {
    matches!(
        action_type,
        "browser_browse" | "computer_screenshot" | "file_read" | "network_search" | "terminal_read"
    )
}

fn agent_action_type_is_permissioned_action(action_type: &str) -> bool {
    matches!(
        action_type,
        "browser_submit"
            | "computer_control"
            | "create_report"
            | "email_draft"
            | "email_send"
            | "file_write"
            | "terminal_write"
    )
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AgentContextReceipt {
    pub id: Uuid,
    pub user_intent: String,
    pub loop_mode: String,
    pub action_type: String,
    pub execution_state: String,
    pub capability: Option<String>,
    pub policy_decision: Option<String>,
    pub capability_invocation_id: Option<Uuid>,
    pub workflow_run_id: Option<Uuid>,
    pub selected_evidence: Vec<String>,
    pub selected_memories: Vec<String>,
    pub model_route: String,
    pub thinking_level: String,
    pub token_cache_state: String,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub validators: Vec<String>,
    #[serde(default)]
    pub stop_conditions: Vec<String>,
    #[serde(default)]
    pub matched_stop_conditions: Vec<String>,
    #[serde(default)]
    pub confirmation_rule: String,
    #[serde(default)]
    pub policy_constraints: Vec<String>,
    pub validation_results: Vec<String>,
    pub intentional_omissions: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl AgentContextReceipt {
    pub fn new(
        action_type: impl Into<String>,
        execution_state: impl Into<String>,
        model_route: impl Into<String>,
        thinking_level: impl Into<String>,
        token_cache_state: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_intent: "Central chat action requested by the user.".to_string(),
            loop_mode: "central_chat_action".to_string(),
            action_type: action_type.into(),
            execution_state: execution_state.into(),
            capability: None,
            policy_decision: None,
            capability_invocation_id: None,
            workflow_run_id: None,
            selected_evidence: Vec::new(),
            selected_memories: Vec::new(),
            model_route: model_route.into(),
            thinking_level: thinking_level.into(),
            token_cache_state: token_cache_state.into(),
            allowed_tools: Vec::new(),
            validators: Vec::new(),
            stop_conditions: Vec::new(),
            matched_stop_conditions: Vec::new(),
            confirmation_rule: String::new(),
            policy_constraints: Vec::new(),
            validation_results: Vec::new(),
            intentional_omissions: Vec::new(),
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{agent_loop_mode_descriptor, classify_agent_action_loop_mode, AgentLoopMode};

    #[test]
    fn classifies_agent_actions_into_explicit_loop_modes() {
        assert_eq!(
            classify_agent_action_loop_mode("file_read", "succeeded", false, false),
            AgentLoopMode::EvidenceGathering
        );
        assert_eq!(
            classify_agent_action_loop_mode("operations_briefing", "succeeded", false, true),
            AgentLoopMode::WorkflowRun
        );
        assert_eq!(
            classify_agent_action_loop_mode("file_write", "needs_confirmation", true, false),
            AgentLoopMode::PermissionedAction
        );
    }

    #[test]
    fn loop_mode_registry_binds_validators_and_stop_conditions() {
        let descriptor = agent_loop_mode_descriptor(AgentLoopMode::EvidenceGathering);

        assert_eq!(descriptor.mode, AgentLoopMode::EvidenceGathering);
        assert!(descriptor.allowed_tools.contains(&"file_read"));
        assert!(descriptor.validators.contains(&"capability_policy_checked"));
        assert!(descriptor
            .validators
            .contains(&"evidence_reference_recorded"));
        assert!(descriptor.stop_conditions.contains(&"evidence_observed"));
        assert_eq!(
            descriptor.confirmation_rule,
            "follow capability policy before tool dispatch"
        );
    }
}
