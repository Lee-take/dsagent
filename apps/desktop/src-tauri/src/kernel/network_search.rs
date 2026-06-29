use serde::{Deserialize, Serialize};

#[cfg(test)]
use crate::kernel::models::LargeModelProvider;
use crate::kernel::models::NetworkSearchBackend;
#[cfg(test)]
use crate::kernel::tool_strategy::model_driven_tool_strategy_for_current_platform;
use crate::kernel::tool_strategy::ModelDrivenToolStrategy;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkSearchExecutionMode {
    PermissionAuditOnly,
    SourceBackedAdapter,
    NativeBridgeContract,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkSearchEvidencePolicy {
    PendingUserConfirmation,
    SourceLinksRequired,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NetworkSearchRouteStatus {
    pub backend: NetworkSearchBackend,
    pub execution_mode: NetworkSearchExecutionMode,
    pub evidence_policy: NetworkSearchEvidencePolicy,
    pub network_requests_enabled: bool,
    pub deepseek_orchestration_ready: bool,
    pub requires_user_confirmation: bool,
    pub note: String,
}

#[cfg(test)]
pub fn network_search_route_status(deepseek_orchestration_ready: bool) -> NetworkSearchRouteStatus {
    let strategy =
        model_driven_tool_strategy_for_current_platform(LargeModelProvider::DeepSeek, None);
    network_search_route_status_for_strategy(&strategy, deepseek_orchestration_ready)
}

pub fn network_search_route_status_for_strategy(
    strategy: &ModelDrivenToolStrategy,
    deepseek_orchestration_ready: bool,
) -> NetworkSearchRouteStatus {
    let native_bridge_ready =
        strategy.network_search_backend == NetworkSearchBackend::NativeLargeModel;
    let source_model_missing = strategy.network_search_source_model_required
        && strategy.network_search_source_model.is_none();

    let source_backed_adapter_ready = strategy.network_search_source_model.is_some();

    NetworkSearchRouteStatus {
        backend: strategy.network_search_backend,
        execution_mode: if native_bridge_ready {
            NetworkSearchExecutionMode::NativeBridgeContract
        } else if source_backed_adapter_ready {
            NetworkSearchExecutionMode::SourceBackedAdapter
        } else {
            NetworkSearchExecutionMode::PermissionAuditOnly
        },
        evidence_policy: if native_bridge_ready || source_backed_adapter_ready {
            NetworkSearchEvidencePolicy::SourceLinksRequired
        } else {
            NetworkSearchEvidencePolicy::PendingUserConfirmation
        },
        network_requests_enabled: native_bridge_ready || source_backed_adapter_ready,
        deepseek_orchestration_ready,
        requires_user_confirmation: source_model_missing,
        note: network_search_route_note(
            strategy,
            source_model_missing,
            native_bridge_ready,
            source_backed_adapter_ready,
        ),
    }
}

fn network_search_route_note(
    strategy: &ModelDrivenToolStrategy,
    source_model_missing: bool,
    native_bridge_ready: bool,
    source_backed_adapter_ready: bool,
) -> String {
    if native_bridge_ready {
        return "NetworkSearch will execute through the selected large model's native bridge contract and requires source links for evidence."
            .to_string();
    }

    if source_backed_adapter_ready {
        return "NetworkSearch will execute through the selected free source-backed adapter and requires source links for evidence."
            .to_string();
    }

    if strategy.large_model_supports_network_search {
        return "NetworkSearch can use the selected large model's source-backed route after a native bridge is connected; local free-source execution requires choosing a source model."
            .to_string();
    }

    if source_model_missing {
        return "The selected large model does not provide source-backed NetworkSearch; choose a free NetworkSearch source model before running search."
            .to_string();
    }

    "NetworkSearch has no executable source-backed adapter selected yet; choose a source model before running search."
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::kernel::models::{
        LargeModelProvider, NetworkSearchBackend, NetworkSearchSourceModel,
    };
    use crate::kernel::tool_strategy::{
        model_driven_tool_strategy, model_driven_tool_strategy_with_native_network_search_bridge,
        RuntimePlatform,
    };

    use super::{
        network_search_route_status, network_search_route_status_for_strategy,
        NetworkSearchEvidencePolicy, NetworkSearchExecutionMode,
    };

    #[test]
    fn network_search_route_status_requires_source_model_for_default_deepseek_route() {
        let status = network_search_route_status(true);

        assert_eq!(status.backend, NetworkSearchBackend::SourceBackedModel);
        assert_eq!(
            status.execution_mode,
            NetworkSearchExecutionMode::PermissionAuditOnly
        );
        assert_eq!(
            status.evidence_policy,
            NetworkSearchEvidencePolicy::PendingUserConfirmation
        );
        assert!(!status.network_requests_enabled);
        assert!(status.deepseek_orchestration_ready);
        assert!(status.requires_user_confirmation);
        assert!(
            status
                .note
                .contains("does not provide source-backed NetworkSearch"),
            "status note should distinguish model orchestration from source-backed search"
        );
    }

    #[test]
    fn network_search_route_status_can_clear_confirmation_when_source_model_selected() {
        let strategy = model_driven_tool_strategy(
            LargeModelProvider::DeepSeek,
            Some(NetworkSearchSourceModel::FreeWebSource),
            RuntimePlatform::Windows,
        );
        let status = network_search_route_status_for_strategy(&strategy, true);

        assert_eq!(status.backend, NetworkSearchBackend::SourceBackedModel);
        assert_eq!(
            status.execution_mode,
            NetworkSearchExecutionMode::SourceBackedAdapter
        );
        assert_eq!(
            status.evidence_policy,
            NetworkSearchEvidencePolicy::SourceLinksRequired
        );
        assert!(status.network_requests_enabled);
        assert!(!status.requires_user_confirmation);
        assert!(status.note.contains("source links"));
    }

    #[test]
    fn network_search_route_status_requires_source_adapter_for_native_model_until_bridge_exists() {
        let strategy =
            model_driven_tool_strategy(LargeModelProvider::ChatGpt, None, RuntimePlatform::Windows);
        let status = network_search_route_status_for_strategy(&strategy, false);

        assert_eq!(status.backend, NetworkSearchBackend::SourceBackedModel);
        assert_eq!(
            status.execution_mode,
            NetworkSearchExecutionMode::PermissionAuditOnly
        );
        assert!(status.requires_user_confirmation);
        assert!(status.note.contains("native bridge"));
    }

    #[test]
    fn network_search_route_status_runs_native_bridge_when_available() {
        let strategy = model_driven_tool_strategy_with_native_network_search_bridge(
            LargeModelProvider::ChatGpt,
            None,
            RuntimePlatform::Windows,
            true,
        );
        let status = network_search_route_status_for_strategy(&strategy, false);

        assert_eq!(status.backend, NetworkSearchBackend::NativeLargeModel);
        assert_eq!(
            status.execution_mode,
            NetworkSearchExecutionMode::NativeBridgeContract
        );
        assert_eq!(
            status.evidence_policy,
            NetworkSearchEvidencePolicy::SourceLinksRequired
        );
        assert!(status.network_requests_enabled);
        assert!(!status.requires_user_confirmation);
        assert!(status.note.contains("native"));
    }

    #[test]
    fn network_search_route_status_serializes_for_ui() {
        let value =
            serde_json::to_value(network_search_route_status(false)).expect("status serializes");

        assert_eq!(value["backend"], "source_backed_model");
        assert_eq!(value["execution_mode"], "permission_audit_only");
        assert_eq!(value["evidence_policy"], "pending_user_confirmation");
        assert_eq!(value["network_requests_enabled"], false);
        assert_eq!(value["deepseek_orchestration_ready"], false);
        assert_eq!(value["requires_user_confirmation"], true);
    }
}
