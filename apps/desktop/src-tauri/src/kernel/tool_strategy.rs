use serde::{Deserialize, Serialize};

use crate::kernel::models::{
    ComputerControlBackend, ComputerScreenshotBackend, LargeModelProvider, NetworkSearchBackend,
    NetworkSearchSourceModel,
};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimePlatform {
    Windows,
    Macos,
    Other,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NetworkSearchSourceModelOption {
    pub value: NetworkSearchSourceModel,
    pub label: String,
    pub note: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModelDrivenToolStrategy {
    pub large_model_provider: LargeModelProvider,
    pub large_model_supports_network_search: bool,
    pub network_search_source_model_required: bool,
    pub network_search_source_model: Option<NetworkSearchSourceModel>,
    pub free_network_search_source_model_options: Vec<NetworkSearchSourceModelOption>,
    pub network_search_backend: NetworkSearchBackend,
    pub computer_screenshot_backend: ComputerScreenshotBackend,
    pub computer_control_backend: ComputerControlBackend,
    pub runtime_platform: RuntimePlatform,
    pub macos_supported: bool,
    pub note: String,
}

impl Default for ModelDrivenToolStrategy {
    fn default() -> Self {
        model_driven_tool_strategy_for_current_platform(LargeModelProvider::DeepSeek, None)
    }
}

pub fn current_runtime_platform() -> RuntimePlatform {
    if cfg!(target_os = "windows") {
        RuntimePlatform::Windows
    } else if cfg!(target_os = "macos") {
        RuntimePlatform::Macos
    } else {
        RuntimePlatform::Other
    }
}

pub fn large_model_supports_network_search(provider: LargeModelProvider) -> bool {
    matches!(provider, LargeModelProvider::ChatGpt)
}

fn native_network_search_bridge_available(provider: LargeModelProvider) -> bool {
    if !large_model_supports_network_search(provider) {
        return false;
    }

    let transport = std::env::var("DEEPSEEK_AGENT_OS_CODEX_BRIDGE_TRANSPORT")
        .ok()
        .map(|value| value.trim().to_string());
    let endpoint_configured = std::env::var("DEEPSEEK_AGENT_OS_CODEX_BRIDGE_URL")
        .ok()
        .is_some_and(|value| !value.trim().is_empty());

    transport.is_some_and(|value| value.eq_ignore_ascii_case("http")) && endpoint_configured
}

pub fn free_network_search_source_model_options() -> Vec<NetworkSearchSourceModelOption> {
    vec![
        NetworkSearchSourceModelOption {
            value: NetworkSearchSourceModel::FreeWebSource,
            label: "Free web source model".to_string(),
            note: "Use a free source-backed web-search adapter for evidence and citations."
                .to_string(),
        },
        NetworkSearchSourceModelOption {
            value: NetworkSearchSourceModel::FreeLocalBrowser,
            label: "Free local browser search (alpha)".to_string(),
            note: "Alpha preset: currently uses the shared source-backed HTTP adapter; reserved for local browser/search-page retrieval."
                .to_string(),
        },
        NetworkSearchSourceModelOption {
            value: NetworkSearchSourceModel::FreeSourceAggregator,
            label: "Free source aggregator (alpha)".to_string(),
            note: "Alpha preset: currently uses the shared source-backed HTTP adapter; reserved for pluggable source aggregation."
                .to_string(),
        },
    ]
}

pub fn model_driven_tool_strategy_for_current_platform(
    large_model_provider: LargeModelProvider,
    network_search_source_model: Option<NetworkSearchSourceModel>,
) -> ModelDrivenToolStrategy {
    model_driven_tool_strategy_with_native_network_search_bridge(
        large_model_provider,
        network_search_source_model,
        current_runtime_platform(),
        native_network_search_bridge_available(large_model_provider),
    )
}

#[cfg(test)]
pub fn model_driven_tool_strategy(
    large_model_provider: LargeModelProvider,
    network_search_source_model: Option<NetworkSearchSourceModel>,
    runtime_platform: RuntimePlatform,
) -> ModelDrivenToolStrategy {
    model_driven_tool_strategy_with_native_network_search_bridge(
        large_model_provider,
        network_search_source_model,
        runtime_platform,
        false,
    )
}

pub fn model_driven_tool_strategy_with_native_network_search_bridge(
    large_model_provider: LargeModelProvider,
    network_search_source_model: Option<NetworkSearchSourceModel>,
    runtime_platform: RuntimePlatform,
    native_network_search_bridge_available: bool,
) -> ModelDrivenToolStrategy {
    let large_model_supports_network_search =
        large_model_supports_network_search(large_model_provider);
    let native_network_search_bridge_available =
        large_model_supports_network_search && native_network_search_bridge_available;
    let network_search_source_model_required =
        !large_model_supports_network_search || !native_network_search_bridge_available;
    let network_search_backend =
        if large_model_supports_network_search && native_network_search_bridge_available {
            NetworkSearchBackend::NativeLargeModel
        } else {
            NetworkSearchBackend::SourceBackedModel
        };
    let network_search_source_model = network_search_source_model_required
        .then_some(network_search_source_model)
        .flatten();
    let (computer_screenshot_backend, computer_control_backend) =
        computer_backends_for(large_model_provider, runtime_platform);
    let note = if network_search_backend == NetworkSearchBackend::NativeLargeModel {
        "NetworkSearch will use the selected large model through the native bridge contract and must preserve source links as evidence."
            .to_string()
    } else if network_search_source_model.is_some() {
        "NetworkSearch will use the selected free source-backed adapter and preserve source links as evidence."
            .to_string()
    } else if large_model_supports_network_search {
        "Selected large model can support NetworkSearch, but the native bridge is not connected in this alpha; choose a free source-backed adapter before running search."
            .to_string()
    } else {
        "Selected large model needs a separate source-backed NetworkSearch model before NetworkSearch can run."
            .to_string()
    };

    ModelDrivenToolStrategy {
        large_model_provider,
        large_model_supports_network_search,
        network_search_source_model_required,
        network_search_source_model,
        free_network_search_source_model_options: free_network_search_source_model_options(),
        network_search_backend,
        computer_screenshot_backend,
        computer_control_backend,
        runtime_platform,
        macos_supported: true,
        note,
    }
}

fn computer_backends_for(
    large_model_provider: LargeModelProvider,
    runtime_platform: RuntimePlatform,
) -> (ComputerScreenshotBackend, ComputerControlBackend) {
    if matches!(
        large_model_provider,
        LargeModelProvider::ChatGpt | LargeModelProvider::Codex
    ) {
        return (
            ComputerScreenshotBackend::CodexBridgeScreenCapture,
            ComputerControlBackend::CodexBridgeInputControl,
        );
    }

    match runtime_platform {
        RuntimePlatform::Macos => (
            ComputerScreenshotBackend::LocalMacosScreenCapture,
            ComputerControlBackend::LocalMacosInputControl,
        ),
        RuntimePlatform::Windows | RuntimePlatform::Other => (
            ComputerScreenshotBackend::LocalWindowsScreenCapture,
            ComputerControlBackend::LocalWindowsInputControl,
        ),
    }
}

#[cfg(test)]
mod tests {
    use crate::kernel::models::{
        ComputerControlBackend, ComputerScreenshotBackend, LargeModelProvider,
        NetworkSearchBackend, NetworkSearchSourceModel,
    };

    use super::{
        free_network_search_source_model_options, model_driven_tool_strategy,
        model_driven_tool_strategy_with_native_network_search_bridge, RuntimePlatform,
    };

    #[test]
    fn deepseek_requires_a_source_backed_network_search_model_on_windows() {
        let strategy = model_driven_tool_strategy(
            LargeModelProvider::DeepSeek,
            None,
            RuntimePlatform::Windows,
        );

        assert!(!strategy.large_model_supports_network_search);
        assert!(strategy.network_search_source_model_required);
        assert_eq!(
            strategy.network_search_backend,
            NetworkSearchBackend::SourceBackedModel
        );
        assert_eq!(
            strategy.computer_screenshot_backend,
            ComputerScreenshotBackend::LocalWindowsScreenCapture
        );
        assert_eq!(
            strategy.computer_control_backend,
            ComputerControlBackend::LocalWindowsInputControl
        );
        assert!(strategy
            .free_network_search_source_model_options
            .iter()
            .any(|option| option.value == NetworkSearchSourceModel::FreeWebSource));
    }

    #[test]
    fn reserved_free_network_search_options_disclose_shared_alpha_adapter() {
        let options = free_network_search_source_model_options();
        let local_browser = options
            .iter()
            .find(|option| option.value == NetworkSearchSourceModel::FreeLocalBrowser)
            .expect("local browser option");
        let source_aggregator = options
            .iter()
            .find(|option| option.value == NetworkSearchSourceModel::FreeSourceAggregator)
            .expect("source aggregator option");

        assert!(local_browser.label.contains("(alpha)"));
        assert!(local_browser.note.contains("Alpha preset"));
        assert!(local_browser
            .note
            .contains("shared source-backed HTTP adapter"));
        assert!(source_aggregator.label.contains("(alpha)"));
        assert!(source_aggregator.note.contains("Alpha preset"));
        assert!(source_aggregator
            .note
            .contains("shared source-backed HTTP adapter"));
    }

    #[test]
    fn chatgpt_uses_source_backed_search_until_native_bridge_is_connected() {
        let strategy =
            model_driven_tool_strategy(LargeModelProvider::ChatGpt, None, RuntimePlatform::Windows);

        assert!(strategy.large_model_supports_network_search);
        assert!(strategy.network_search_source_model_required);
        assert_eq!(
            strategy.network_search_backend,
            NetworkSearchBackend::SourceBackedModel
        );
        assert_eq!(
            strategy.computer_screenshot_backend,
            ComputerScreenshotBackend::CodexBridgeScreenCapture
        );
        assert_eq!(
            strategy.computer_control_backend,
            ComputerControlBackend::CodexBridgeInputControl
        );
    }

    #[test]
    fn chatgpt_can_run_network_search_with_free_source_model_in_alpha() {
        let strategy = model_driven_tool_strategy(
            LargeModelProvider::ChatGpt,
            Some(NetworkSearchSourceModel::FreeWebSource),
            RuntimePlatform::Windows,
        );

        assert!(strategy.large_model_supports_network_search);
        assert!(strategy.network_search_source_model_required);
        assert_eq!(
            strategy.network_search_source_model,
            Some(NetworkSearchSourceModel::FreeWebSource)
        );
        assert_eq!(
            strategy.network_search_backend,
            NetworkSearchBackend::SourceBackedModel
        );
    }

    #[test]
    fn chatgpt_uses_native_network_search_when_bridge_is_available() {
        let strategy = model_driven_tool_strategy_with_native_network_search_bridge(
            LargeModelProvider::ChatGpt,
            None,
            RuntimePlatform::Windows,
            true,
        );

        assert!(strategy.large_model_supports_network_search);
        assert!(!strategy.network_search_source_model_required);
        assert_eq!(strategy.network_search_source_model, None);
        assert_eq!(
            strategy.network_search_backend,
            NetworkSearchBackend::NativeLargeModel
        );
        assert!(strategy.note.contains("native"));
    }

    #[test]
    fn codex_uses_codex_bridge_but_still_requires_source_backed_search() {
        let strategy =
            model_driven_tool_strategy(LargeModelProvider::Codex, None, RuntimePlatform::Windows);

        assert!(!strategy.large_model_supports_network_search);
        assert!(strategy.network_search_source_model_required);
        assert_eq!(
            strategy.network_search_backend,
            NetworkSearchBackend::SourceBackedModel
        );
        assert_eq!(
            strategy.computer_screenshot_backend,
            ComputerScreenshotBackend::CodexBridgeScreenCapture
        );
        assert_eq!(
            strategy.computer_control_backend,
            ComputerControlBackend::CodexBridgeInputControl
        );
    }

    #[test]
    fn non_bridge_models_on_macos_use_local_macos_computer_backends() {
        let strategy = model_driven_tool_strategy(
            LargeModelProvider::Custom,
            Some(NetworkSearchSourceModel::FreeWebSource),
            RuntimePlatform::Macos,
        );

        assert_eq!(
            strategy.computer_screenshot_backend,
            ComputerScreenshotBackend::LocalMacosScreenCapture
        );
        assert_eq!(
            strategy.computer_control_backend,
            ComputerControlBackend::LocalMacosInputControl
        );
        assert!(strategy.macos_supported);
    }
}
