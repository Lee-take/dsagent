use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::kernel::codex_bridge_contract::{
    CodexBridgeCapability, CodexBridgeHealthResponse, CODEX_BRIDGE_CONTRACT_VERSION,
};
use crate::kernel::codex_bridge_http::CodexBridgeHttpClient;
#[cfg(test)]
use crate::kernel::models::LargeModelProvider;
use crate::kernel::models::{ComputerControlBackend, ComputerScreenshotBackend};
#[cfg(test)]
use crate::kernel::tool_strategy::model_driven_tool_strategy_for_current_platform;
use crate::kernel::tool_strategy::ModelDrivenToolStrategy;

pub const CODEX_BRIDGE_ENDPOINT_ENV_VAR: &str = "DEEPSEEK_AGENT_OS_CODEX_BRIDGE_URL";
pub const CODEX_BRIDGE_TRANSPORT_ENV_VAR: &str = "DEEPSEEK_AGENT_OS_CODEX_BRIDGE_TRANSPORT";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CodexBridgeTransport {
    Http,
    Stdio,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CodexBridgeTransportOption {
    pub value: CodexBridgeTransport,
    pub label: String,
    pub note: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CodexBridgeRuntimeStatus {
    pub required: bool,
    pub transport_env_var: String,
    #[serde(default)]
    pub transport: Option<CodexBridgeTransport>,
    #[serde(default)]
    pub transport_decision_required: bool,
    #[serde(default = "codex_bridge_transport_options")]
    pub transport_options: Vec<CodexBridgeTransportOption>,
    pub endpoint_env_var: String,
    pub endpoint_configured: bool,
    pub connected: bool,
    pub note: String,
}

impl Default for CodexBridgeRuntimeStatus {
    fn default() -> Self {
        Self {
            required: false,
            transport_env_var: CODEX_BRIDGE_TRANSPORT_ENV_VAR.to_string(),
            transport: None,
            transport_decision_required: false,
            transport_options: codex_bridge_transport_options(),
            endpoint_env_var: CODEX_BRIDGE_ENDPOINT_ENV_VAR.to_string(),
            endpoint_configured: false,
            connected: false,
            note: String::new(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ComputerUseBackendStatus {
    pub screenshot_backend: ComputerScreenshotBackend,
    pub screenshot_available: bool,
    pub screenshot_note: String,
    #[serde(default)]
    pub screenshot_permission_required: bool,
    #[serde(default)]
    pub screenshot_permission_note: String,
    pub control_backend: ComputerControlBackend,
    pub control_available: bool,
    pub control_requires_approval: bool,
    pub control_note: String,
    #[serde(default)]
    pub control_permission_required: bool,
    #[serde(default)]
    pub control_permission_note: String,
    #[serde(default)]
    pub codex_bridge: CodexBridgeRuntimeStatus,
}

#[cfg(test)]
pub fn computer_use_backend_status() -> ComputerUseBackendStatus {
    let strategy =
        model_driven_tool_strategy_for_current_platform(LargeModelProvider::DeepSeek, None);
    computer_use_backend_status_for_strategy(&strategy)
}

pub fn computer_use_backend_status_for_strategy(
    strategy: &ModelDrivenToolStrategy,
) -> ComputerUseBackendStatus {
    let endpoint = std::env::var(CODEX_BRIDGE_ENDPOINT_ENV_VAR).ok();
    let transport = std::env::var(CODEX_BRIDGE_TRANSPORT_ENV_VAR).ok();
    computer_use_backend_status_for_strategy_with_codex_bridge_config(
        strategy,
        endpoint.as_deref(),
        transport.as_deref(),
    )
}

#[cfg(test)]
pub fn computer_use_backend_status_for_strategy_with_codex_bridge_endpoint(
    strategy: &ModelDrivenToolStrategy,
    codex_bridge_endpoint: Option<&str>,
) -> ComputerUseBackendStatus {
    computer_use_backend_status_for_strategy_with_codex_bridge_config(
        strategy,
        codex_bridge_endpoint,
        None,
    )
}

pub fn computer_use_backend_status_for_strategy_with_codex_bridge_config(
    strategy: &ModelDrivenToolStrategy,
    codex_bridge_endpoint: Option<&str>,
    codex_bridge_transport: Option<&str>,
) -> ComputerUseBackendStatus {
    let codex_bridge =
        codex_bridge_runtime_status(strategy, codex_bridge_endpoint, codex_bridge_transport);
    let screenshot_available = matches!(
        strategy.computer_screenshot_backend,
        ComputerScreenshotBackend::LocalWindowsScreenCapture
            | ComputerScreenshotBackend::LocalMacosScreenCapture
    ) || (matches!(
        strategy.computer_screenshot_backend,
        ComputerScreenshotBackend::CodexBridgeScreenCapture
    ) && codex_bridge.connected);
    let control_available = matches!(
        strategy.computer_control_backend,
        ComputerControlBackend::LocalWindowsInputControl
            | ComputerControlBackend::LocalMacosInputControl
    ) || (matches!(
        strategy.computer_control_backend,
        ComputerControlBackend::CodexBridgeInputControl
    ) && codex_bridge.connected);

    ComputerUseBackendStatus {
        screenshot_backend: strategy.computer_screenshot_backend,
        screenshot_available,
        screenshot_note: computer_screenshot_note(strategy, codex_bridge.connected),
        screenshot_permission_required: computer_screenshot_permission_required(strategy),
        screenshot_permission_note: computer_screenshot_permission_note(strategy),
        control_backend: strategy.computer_control_backend,
        control_available,
        control_requires_approval: true,
        control_note: computer_control_note(strategy, codex_bridge.connected),
        control_permission_required: computer_control_permission_required(strategy),
        control_permission_note: computer_control_permission_note(strategy),
        codex_bridge,
    }
}

fn codex_bridge_runtime_status(
    strategy: &ModelDrivenToolStrategy,
    codex_bridge_endpoint: Option<&str>,
    codex_bridge_transport: Option<&str>,
) -> CodexBridgeRuntimeStatus {
    let required = matches!(
        strategy.computer_screenshot_backend,
        ComputerScreenshotBackend::CodexBridgeScreenCapture
    ) || matches!(
        strategy.computer_control_backend,
        ComputerControlBackend::CodexBridgeInputControl
    );
    let endpoint_configured = codex_bridge_endpoint
        .map(str::trim)
        .is_some_and(|endpoint| !endpoint.is_empty());
    let transport = codex_bridge_transport.and_then(parse_codex_bridge_transport);
    let transport_decision_required = required && transport.is_none();
    let http_health = if required
        && matches!(transport, Some(CodexBridgeTransport::Http))
        && endpoint_configured
    {
        codex_bridge_endpoint.map(|endpoint| codex_bridge_http_health_status(strategy, endpoint))
    } else {
        None
    };
    let connected = http_health
        .as_ref()
        .map(|health| health.connected)
        .unwrap_or(false);
    let note = if !required {
        "Codex bridge runtime is not required for the selected local Computer Use route."
            .to_string()
    } else if transport_decision_required {
        format!(
            "Select a Codex bridge transport with {CODEX_BRIDGE_TRANSPORT_ENV_VAR}=http or stdio before ChatGPT/Codex Computer Use can run."
        )
    } else if matches!(transport, Some(CodexBridgeTransport::Http)) && !endpoint_configured {
        format!(
            "Set {CODEX_BRIDGE_ENDPOINT_ENV_VAR} to a local HTTP Codex bridge endpoint before ChatGPT/Codex Computer Use can run."
        )
    } else if let Some(health) = http_health {
        health.note
    } else if matches!(transport, Some(CodexBridgeTransport::Stdio)) {
        "Codex bridge stdio sidecar is deferred in this MVP; use an external loopback HTTP bridge runtime instead."
            .to_string()
    } else {
        format!(
            "Set {CODEX_BRIDGE_ENDPOINT_ENV_VAR} to a local Codex bridge endpoint before ChatGPT/Codex Computer Use can run."
        )
    };

    CodexBridgeRuntimeStatus {
        required,
        transport_env_var: CODEX_BRIDGE_TRANSPORT_ENV_VAR.to_string(),
        transport,
        transport_decision_required,
        transport_options: codex_bridge_transport_options(),
        endpoint_env_var: CODEX_BRIDGE_ENDPOINT_ENV_VAR.to_string(),
        endpoint_configured,
        connected,
        note,
    }
}

pub fn codex_bridge_transport_options() -> Vec<CodexBridgeTransportOption> {
    vec![CodexBridgeTransportOption {
        value: CodexBridgeTransport::Http,
        label: "External HTTP bridge".to_string(),
        note: "Use an external loopback HTTP service with health, screenshot, control, and network-search endpoints.".to_string(),
    }]
}

struct CodexBridgeHealthProbe {
    connected: bool,
    note: String,
}

fn codex_bridge_http_health_status(
    strategy: &ModelDrivenToolStrategy,
    endpoint: &str,
) -> CodexBridgeHealthProbe {
    match check_codex_bridge_http_health(endpoint) {
        Ok(health) => codex_bridge_health_probe_from_response(strategy, health),
        Err(error) => CodexBridgeHealthProbe {
            connected: false,
            note: format!("Codex bridge HTTP health check failed: {error}"),
        },
    }
}

fn check_codex_bridge_http_health(endpoint: &str) -> Result<CodexBridgeHealthResponse, String> {
    CodexBridgeHttpClient::new(endpoint, Duration::from_millis(750))?.health()
}

fn codex_bridge_health_probe_from_response(
    strategy: &ModelDrivenToolStrategy,
    health: CodexBridgeHealthResponse,
) -> CodexBridgeHealthProbe {
    if health.contract_version != CODEX_BRIDGE_CONTRACT_VERSION {
        return CodexBridgeHealthProbe {
            connected: false,
            note: format!(
                "Codex bridge HTTP health check returned contract version {}, expected {}.",
                health.contract_version, CODEX_BRIDGE_CONTRACT_VERSION
            ),
        };
    }

    let missing = required_codex_bridge_capabilities(strategy)
        .into_iter()
        .filter(|capability| !health_capability_available(&health, *capability))
        .map(codex_bridge_capability_name)
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        return CodexBridgeHealthProbe {
            connected: false,
            note: format!(
                "Codex bridge HTTP health check reached {} {}, but required capabilities are unavailable: {}.",
                health.runtime_name,
                health.runtime_version,
                missing.join(", ")
            ),
        };
    }

    CodexBridgeHealthProbe {
        connected: true,
        note: format!(
            "Codex bridge HTTP health check connected to {} {}.",
            health.runtime_name, health.runtime_version
        ),
    }
}

fn required_codex_bridge_capabilities(
    strategy: &ModelDrivenToolStrategy,
) -> Vec<CodexBridgeCapability> {
    let mut capabilities = Vec::new();
    if matches!(
        strategy.computer_screenshot_backend,
        ComputerScreenshotBackend::CodexBridgeScreenCapture
    ) {
        capabilities.push(CodexBridgeCapability::ComputerScreenshot);
    }
    if matches!(
        strategy.computer_control_backend,
        ComputerControlBackend::CodexBridgeInputControl
    ) {
        capabilities.push(CodexBridgeCapability::ComputerControl);
    }

    capabilities
}

fn health_capability_available(
    health: &CodexBridgeHealthResponse,
    capability: CodexBridgeCapability,
) -> bool {
    health
        .capabilities
        .iter()
        .any(|status| status.capability == capability && status.available)
}

fn codex_bridge_capability_name(capability: CodexBridgeCapability) -> &'static str {
    match capability {
        CodexBridgeCapability::ComputerScreenshot => "computer_screenshot",
        CodexBridgeCapability::ComputerControl => "computer_control",
        CodexBridgeCapability::NetworkSearch => "network_search",
    }
}

fn parse_codex_bridge_transport(value: &str) -> Option<CodexBridgeTransport> {
    match value.trim().to_ascii_lowercase().as_str() {
        "http" => Some(CodexBridgeTransport::Http),
        "stdio" => Some(CodexBridgeTransport::Stdio),
        _ => None,
    }
}

fn computer_screenshot_note(
    strategy: &ModelDrivenToolStrategy,
    codex_bridge_connected: bool,
) -> String {
    match strategy.computer_screenshot_backend {
        ComputerScreenshotBackend::CodexBridgeScreenCapture if codex_bridge_connected => {
            "screen pixels are routed through the connected Codex bridge contract for the selected large model"
                .to_string()
        }
        ComputerScreenshotBackend::CodexBridgeScreenCapture => {
            "screen pixels are routed through the Codex bridge contract for the selected large model, but the bridge runtime is not connected"
                .to_string()
        }
        ComputerScreenshotBackend::LocalWindowsScreenCapture => {
            "screen pixels are routed through the local Windows screen capture library".to_string()
        }
        ComputerScreenshotBackend::LocalMacosScreenCapture => {
            "screen pixels are routed through the local macOS screen capture library".to_string()
        }
        ComputerScreenshotBackend::CodexStyleScreenCapture => {
            "legacy codex-style screen capture backend is configured but not connected".to_string()
        }
    }
}

fn computer_screenshot_permission_required(strategy: &ModelDrivenToolStrategy) -> bool {
    matches!(
        strategy.computer_screenshot_backend,
        ComputerScreenshotBackend::LocalMacosScreenCapture
    )
}

fn computer_screenshot_permission_note(strategy: &ModelDrivenToolStrategy) -> String {
    match strategy.computer_screenshot_backend {
        ComputerScreenshotBackend::LocalMacosScreenCapture => {
            "macOS requires Screen Recording permission for local screen pixel capture."
                .to_string()
        }
        ComputerScreenshotBackend::LocalWindowsScreenCapture => {
            "Local Windows desktop capture usually runs without a separate OS permission prompt, but secure desktops and protected windows can block pixels."
                .to_string()
        }
        ComputerScreenshotBackend::CodexBridgeScreenCapture => {
            "Connect a Codex bridge runtime before requesting bridge-routed screen pixels."
                .to_string()
        }
        ComputerScreenshotBackend::CodexStyleScreenCapture => {
            "Connect the legacy Codex-style screen capture runtime before requesting screen pixels."
                .to_string()
        }
    }
}

fn computer_control_note(
    strategy: &ModelDrivenToolStrategy,
    codex_bridge_connected: bool,
) -> String {
    match strategy.computer_control_backend {
        ComputerControlBackend::CodexBridgeInputControl if codex_bridge_connected => {
            "mouse and keyboard control is routed through the connected Codex bridge contract for the selected large model"
                .to_string()
        }
        ComputerControlBackend::CodexBridgeInputControl => {
            "mouse and keyboard control is routed through the Codex bridge contract for the selected large model, but the bridge runtime is not connected"
                .to_string()
        }
        ComputerControlBackend::LocalWindowsInputControl => {
            "mouse and keyboard control is routed through the local Windows input library"
                .to_string()
        }
        ComputerControlBackend::LocalMacosInputControl => {
            "mouse and keyboard control is routed through the local macOS input library".to_string()
        }
        ComputerControlBackend::CodexStyleInputControl => {
            "legacy codex-style mouse and keyboard backend is configured but not connected"
                .to_string()
        }
    }
}

fn computer_control_permission_required(strategy: &ModelDrivenToolStrategy) -> bool {
    matches!(
        strategy.computer_control_backend,
        ComputerControlBackend::LocalMacosInputControl
    )
}

fn computer_control_permission_note(strategy: &ModelDrivenToolStrategy) -> String {
    match strategy.computer_control_backend {
        ComputerControlBackend::LocalMacosInputControl => {
            "macOS requires Accessibility permission before local mouse and keyboard control can run."
                .to_string()
        }
        ComputerControlBackend::LocalWindowsInputControl => {
            "Local Windows input control runs against the foreground desktop and can be blocked by secure desktop prompts or elevated target windows."
                .to_string()
        }
        ComputerControlBackend::CodexBridgeInputControl => {
            "Connect a Codex bridge runtime before requesting bridge-routed mouse and keyboard control."
                .to_string()
        }
        ComputerControlBackend::CodexStyleInputControl => {
            "Connect the legacy Codex-style input runtime before requesting mouse and keyboard control."
                .to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread::JoinHandle;
    use std::time::{Duration, Instant};

    use crate::kernel::models::{
        ComputerControlBackend, ComputerScreenshotBackend, LargeModelProvider,
    };
    use crate::kernel::tool_strategy::{model_driven_tool_strategy, RuntimePlatform};

    use super::{computer_use_backend_status, computer_use_backend_status_for_strategy};

    struct RecordedHttpRequest {
        raw: String,
    }

    fn serve_one_json_response(response_body: String) -> (String, JoinHandle<RecordedHttpRequest>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind fake bridge server");
        listener
            .set_nonblocking(true)
            .expect("fake bridge server nonblocking");
        let endpoint = format!("http://{}", listener.local_addr().expect("local addr"));
        let handle = std::thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(2);
            let mut stream = loop {
                match listener.accept() {
                    Ok((stream, _)) => break stream,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        if Instant::now() >= deadline {
                            return RecordedHttpRequest { raw: String::new() };
                        }
                        std::thread::sleep(Duration::from_millis(10));
                    }
                    Err(error) => panic!("accept fake bridge request: {error}"),
                }
            };
            stream
                .set_nonblocking(false)
                .expect("fake bridge request stream blocking");
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .expect("set read timeout");
            let raw = read_one_http_request(&mut stream);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                response_body.len(),
                response_body
            );
            stream
                .write_all(response.as_bytes())
                .expect("write fake bridge response");
            RecordedHttpRequest { raw }
        });

        (endpoint, handle)
    }

    fn read_one_http_request(stream: &mut std::net::TcpStream) -> String {
        let mut request = Vec::new();
        let mut buffer = [0_u8; 1024];
        loop {
            let bytes_read = stream.read(&mut buffer).expect("read request chunk");
            if bytes_read == 0 {
                break;
            }
            request.extend_from_slice(&buffer[..bytes_read]);
            if http_request_complete(&request) {
                break;
            }
        }

        String::from_utf8(request).expect("request is utf8")
    }

    fn http_request_complete(request: &[u8]) -> bool {
        let Some(headers_end) = request.windows(4).position(|window| window == b"\r\n\r\n") else {
            return false;
        };
        let headers = String::from_utf8_lossy(&request[..headers_end]);
        let content_length = headers
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().ok())
                    .flatten()
            })
            .unwrap_or(0);

        request.len() >= headers_end + 4 + content_length
    }

    #[test]
    fn computer_use_backend_status_reports_default_local_backends() {
        let status = computer_use_backend_status();

        if cfg!(target_os = "macos") {
            assert_eq!(
                status.screenshot_backend,
                ComputerScreenshotBackend::LocalMacosScreenCapture
            );
            assert_eq!(
                status.control_backend,
                ComputerControlBackend::LocalMacosInputControl
            );
        } else {
            assert_eq!(
                status.screenshot_backend,
                ComputerScreenshotBackend::LocalWindowsScreenCapture
            );
            assert_eq!(
                status.control_backend,
                ComputerControlBackend::LocalWindowsInputControl
            );
        }
        assert!(status.screenshot_available);
        assert!(status.control_available);
        assert!(status.control_requires_approval);
    }

    #[test]
    fn computer_use_backend_status_can_report_codex_bridge_for_chatgpt() {
        let strategy =
            model_driven_tool_strategy(LargeModelProvider::ChatGpt, None, RuntimePlatform::Windows);
        let status = super::computer_use_backend_status_for_strategy_with_codex_bridge_config(
            &strategy, None, None,
        );

        assert_eq!(
            status.screenshot_backend,
            ComputerScreenshotBackend::CodexBridgeScreenCapture
        );
        assert_eq!(
            status.control_backend,
            ComputerControlBackend::CodexBridgeInputControl
        );
        assert!(!status.screenshot_available);
        assert!(!status.control_available);
        assert!(
            status.control_note.contains("not connected"),
            "control note should explain the bridge is not connected"
        );
    }

    #[test]
    fn codex_bridge_http_status_requires_endpoint_for_chatgpt_route() {
        let strategy =
            model_driven_tool_strategy(LargeModelProvider::ChatGpt, None, RuntimePlatform::Windows);

        let status = super::computer_use_backend_status_for_strategy_with_codex_bridge_config(
            &strategy,
            None,
            Some("http"),
        );

        assert!(status.codex_bridge.required);
        assert_eq!(
            status.codex_bridge.transport,
            Some(super::CodexBridgeTransport::Http)
        );
        assert!(!status.codex_bridge.transport_decision_required);
        assert!(!status.codex_bridge.endpoint_configured);
        assert!(!status.codex_bridge.connected);
        assert_eq!(
            status.codex_bridge.endpoint_env_var,
            super::CODEX_BRIDGE_ENDPOINT_ENV_VAR
        );
        assert!(status
            .codex_bridge
            .note
            .contains(super::CODEX_BRIDGE_ENDPOINT_ENV_VAR));
    }

    #[test]
    fn codex_bridge_status_requires_transport_decision_before_execution() {
        let strategy =
            model_driven_tool_strategy(LargeModelProvider::ChatGpt, None, RuntimePlatform::Windows);

        let status = super::computer_use_backend_status_for_strategy_with_codex_bridge_config(
            &strategy, None, None,
        );

        assert!(status.codex_bridge.required);
        assert!(status.codex_bridge.transport_decision_required);
        assert_eq!(status.codex_bridge.transport, None);
        assert!(status
            .codex_bridge
            .transport_env_var
            .contains("CODEX_BRIDGE_TRANSPORT"));
        assert!(status
            .codex_bridge
            .transport_options
            .iter()
            .any(|option| option.value == super::CodexBridgeTransport::Http));
        assert!(!status
            .codex_bridge
            .transport_options
            .iter()
            .any(|option| option.value == super::CodexBridgeTransport::Stdio));
    }

    #[test]
    fn codex_bridge_status_marks_endpoint_configured_without_claiming_connected() {
        let strategy =
            model_driven_tool_strategy(LargeModelProvider::Codex, None, RuntimePlatform::Windows);

        let status = super::computer_use_backend_status_for_strategy_with_codex_bridge_config(
            &strategy,
            Some("http://127.0.0.1:47329"),
            Some("http"),
        );

        assert!(status.codex_bridge.required);
        assert_eq!(
            status.codex_bridge.transport,
            Some(super::CodexBridgeTransport::Http)
        );
        assert!(!status.codex_bridge.transport_decision_required);
        assert!(status.codex_bridge.endpoint_configured);
        assert!(!status.codex_bridge.connected);
        assert!(status.codex_bridge.note.contains("health check"));
    }

    #[test]
    fn codex_bridge_http_status_marks_connected_when_health_reports_required_capabilities() {
        let strategy =
            model_driven_tool_strategy(LargeModelProvider::Codex, None, RuntimePlatform::Windows);
        let response_body = serde_json::json!({
            "contract_version": "deepseek-agent-os.codex-bridge.v1",
            "runtime_name": "fake-codex-bridge",
            "runtime_version": "0.1.0",
            "capabilities": [
                {
                    "capability": "computer_screenshot",
                    "available": true,
                    "note": "ready"
                },
                {
                    "capability": "computer_control",
                    "available": true,
                    "note": "ready"
                }
            ]
        })
        .to_string();
        let (endpoint, handle) = serve_one_json_response(response_body);

        let status = super::computer_use_backend_status_for_strategy_with_codex_bridge_config(
            &strategy,
            Some(&endpoint),
            Some("http"),
        );
        let recorded = handle.join().expect("fake bridge thread joins");

        assert!(recorded.raw.starts_with("POST /health HTTP/1.1"));
        assert!(status.codex_bridge.required);
        assert!(status.codex_bridge.endpoint_configured);
        assert!(status.codex_bridge.connected);
        assert!(status.screenshot_available);
        assert!(status.control_available);
        assert!(!status.screenshot_note.contains("not connected"));
        assert!(!status.control_note.contains("not connected"));
        assert!(status.codex_bridge.note.contains("fake-codex-bridge"));
    }

    #[test]
    fn codex_bridge_status_marks_stdio_sidecar_as_deferred() {
        let strategy =
            model_driven_tool_strategy(LargeModelProvider::Codex, None, RuntimePlatform::Windows);

        let status = super::computer_use_backend_status_for_strategy_with_codex_bridge_config(
            &strategy,
            None,
            Some("stdio"),
        );

        assert!(status.codex_bridge.required);
        assert_eq!(
            status.codex_bridge.transport,
            Some(super::CodexBridgeTransport::Stdio)
        );
        assert!(!status.codex_bridge.transport_decision_required);
        assert!(!status.codex_bridge.endpoint_configured);
        assert!(!status.codex_bridge.connected);
        assert!(status.codex_bridge.note.contains("deferred"));
    }

    #[test]
    fn local_computer_use_route_does_not_require_codex_bridge() {
        let strategy = model_driven_tool_strategy(
            LargeModelProvider::DeepSeek,
            None,
            RuntimePlatform::Windows,
        );

        let status = super::computer_use_backend_status_for_strategy_with_codex_bridge_endpoint(
            &strategy,
            Some("http://127.0.0.1:47329"),
        );

        assert!(!status.codex_bridge.required);
        assert!(!status.codex_bridge.connected);
    }

    #[test]
    fn local_macos_computer_use_status_reports_os_permission_prompts() {
        let strategy =
            model_driven_tool_strategy(LargeModelProvider::DeepSeek, None, RuntimePlatform::Macos);

        let status = computer_use_backend_status_for_strategy(&strategy);

        assert!(status.screenshot_permission_required);
        assert!(status
            .screenshot_permission_note
            .contains("Screen Recording"));
        assert!(status.control_permission_required);
        assert!(status.control_permission_note.contains("Accessibility"));
    }

    #[test]
    fn local_windows_computer_use_status_reports_foreground_desktop_notes() {
        let strategy = model_driven_tool_strategy(
            LargeModelProvider::DeepSeek,
            None,
            RuntimePlatform::Windows,
        );

        let status = computer_use_backend_status_for_strategy(&strategy);

        assert!(!status.screenshot_permission_required);
        assert!(status
            .screenshot_permission_note
            .contains("Windows desktop"));
        assert!(!status.control_permission_required);
        assert!(status
            .control_permission_note
            .contains("foreground desktop"));
    }

    #[test]
    fn legacy_computer_use_backend_status_json_defaults_permission_fields() {
        let status = serde_json::from_value::<super::ComputerUseBackendStatus>(serde_json::json!({
            "screenshot_backend": "local_windows_screen_capture",
            "screenshot_available": true,
            "screenshot_note": "screen pixels are routed through the local Windows screen capture library",
            "control_backend": "local_windows_input_control",
            "control_available": true,
            "control_requires_approval": true,
            "control_note": "mouse and keyboard control is routed through the local Windows input library"
        }))
        .expect("legacy status parses");

        assert!(!status.screenshot_permission_required);
        assert_eq!(status.screenshot_permission_note, "");
        assert!(!status.control_permission_required);
        assert_eq!(status.control_permission_note, "");
        assert!(!status.codex_bridge.required);
        assert!(!status.codex_bridge.transport_decision_required);
        assert_eq!(status.codex_bridge.transport, None);
        assert!(!status.codex_bridge.endpoint_configured);
        assert!(!status.codex_bridge.connected);
    }

    #[test]
    fn computer_use_backend_status_serializes_backend_names_for_ui() {
        let value = serde_json::to_value(computer_use_backend_status()).expect("status serializes");

        if cfg!(target_os = "macos") {
            assert_eq!(value["screenshot_backend"], "local_macos_screen_capture");
            assert_eq!(value["control_backend"], "local_macos_input_control");
        } else {
            assert_eq!(value["screenshot_backend"], "local_windows_screen_capture");
            assert_eq!(value["control_backend"], "local_windows_input_control");
        }
        assert_eq!(value["control_requires_approval"], true);
    }
}
