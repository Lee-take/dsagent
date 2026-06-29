#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::kernel::models::LargeModelProvider;

pub const CODEX_BRIDGE_CONTRACT_VERSION: &str = "deepseek-agent-os.codex-bridge.v1";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CodexBridgeCapability {
    ComputerScreenshot,
    ComputerControl,
    NetworkSearch,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CodexBridgeHealthRequest {
    pub contract_version: String,
    pub requested_capabilities: Vec<CodexBridgeCapability>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CodexBridgeCapabilityStatus {
    pub capability: CodexBridgeCapability,
    pub available: bool,
    pub note: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CodexBridgeHealthResponse {
    pub contract_version: String,
    pub runtime_name: String,
    pub runtime_version: String,
    pub capabilities: Vec<CodexBridgeCapabilityStatus>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CodexBridgeScreenshotRequest {
    pub contract_version: String,
    pub capability: CodexBridgeCapability,
    pub display_hint: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CodexBridgeScreenshotResponse {
    pub contract_version: String,
    pub capability: CodexBridgeCapability,
    pub display_label: String,
    pub width: u32,
    pub height: u32,
    pub png_base64: String,
    pub captured_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CodexBridgeControlRequest {
    pub contract_version: String,
    pub capability: CodexBridgeCapability,
    pub target: String,
    pub action: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CodexBridgeControlResponse {
    pub contract_version: String,
    pub capability: CodexBridgeCapability,
    pub summary: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CodexBridgeNetworkSearchRequest {
    pub contract_version: String,
    pub capability: CodexBridgeCapability,
    pub large_model_provider: LargeModelProvider,
    pub query: String,
    pub scope: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CodexBridgeNetworkSearchItem {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CodexBridgeNetworkSearchResponse {
    pub contract_version: String,
    pub capability: CodexBridgeCapability,
    pub provider: String,
    pub query: String,
    pub scope: String,
    pub search_url: String,
    pub items: Vec<CodexBridgeNetworkSearchItem>,
}

pub fn default_codex_bridge_health_request() -> CodexBridgeHealthRequest {
    CodexBridgeHealthRequest {
        contract_version: CODEX_BRIDGE_CONTRACT_VERSION.to_string(),
        requested_capabilities: vec![
            CodexBridgeCapability::ComputerScreenshot,
            CodexBridgeCapability::ComputerControl,
            CodexBridgeCapability::NetworkSearch,
        ],
    }
}

impl CodexBridgeScreenshotRequest {
    pub fn new(display_hint: Option<String>) -> Self {
        Self {
            contract_version: CODEX_BRIDGE_CONTRACT_VERSION.to_string(),
            capability: CodexBridgeCapability::ComputerScreenshot,
            display_hint,
        }
    }
}

impl CodexBridgeScreenshotResponse {
    pub fn new(
        display_label: &str,
        width: u32,
        height: u32,
        png_base64: &str,
        captured_at: DateTime<Utc>,
    ) -> Result<Self, String> {
        if width == 0 || height == 0 {
            return Err("codex bridge screenshot response requires nonzero dimensions".to_string());
        }
        let display_label = display_label.trim();
        if display_label.is_empty() {
            return Err("codex bridge screenshot response requires a display label".to_string());
        }
        let png_base64 = png_base64.trim();
        if png_base64.is_empty() {
            return Err("codex bridge screenshot response requires PNG base64".to_string());
        }

        Ok(Self {
            contract_version: CODEX_BRIDGE_CONTRACT_VERSION.to_string(),
            capability: CodexBridgeCapability::ComputerScreenshot,
            display_label: display_label.to_string(),
            width,
            height,
            png_base64: png_base64.to_string(),
            captured_at,
        })
    }
}

impl CodexBridgeControlRequest {
    pub fn new(target: &str, action: &str) -> Result<Self, String> {
        let target = target.trim();
        if target.is_empty() {
            return Err("codex bridge control request requires a target".to_string());
        }
        let action = action.trim();
        if !is_structured_control_action(action) {
            return Err("codex bridge control request requires a structured action".to_string());
        }

        Ok(Self {
            contract_version: CODEX_BRIDGE_CONTRACT_VERSION.to_string(),
            capability: CodexBridgeCapability::ComputerControl,
            target: target.to_string(),
            action: action.to_string(),
        })
    }
}

impl CodexBridgeControlResponse {
    pub fn new(summary: &str) -> Result<Self, String> {
        let summary = summary.trim();
        if summary.is_empty() {
            return Err("codex bridge control response requires a summary".to_string());
        }

        Ok(Self {
            contract_version: CODEX_BRIDGE_CONTRACT_VERSION.to_string(),
            capability: CodexBridgeCapability::ComputerControl,
            summary: summary.to_string(),
        })
    }
}

impl CodexBridgeNetworkSearchRequest {
    pub fn new(
        large_model_provider: LargeModelProvider,
        query: &str,
        scope: &str,
    ) -> Result<Self, String> {
        let query = query.trim();
        if query.is_empty() {
            return Err("codex bridge network search request requires a query".to_string());
        }
        let scope = scope.trim();
        let scope = if scope.is_empty() {
            "public web"
        } else {
            scope
        };

        Ok(Self {
            contract_version: CODEX_BRIDGE_CONTRACT_VERSION.to_string(),
            capability: CodexBridgeCapability::NetworkSearch,
            large_model_provider,
            query: query.to_string(),
            scope: scope.to_string(),
        })
    }
}

impl CodexBridgeNetworkSearchResponse {
    pub fn new(
        provider: &str,
        query: &str,
        scope: &str,
        search_url: &str,
        items: Vec<CodexBridgeNetworkSearchItem>,
    ) -> Result<Self, String> {
        let provider = provider.trim();
        if provider.is_empty() {
            return Err("codex bridge network search response requires a provider".to_string());
        }
        let query = query.trim();
        if query.is_empty() {
            return Err("codex bridge network search response requires a query".to_string());
        }
        let scope = scope.trim();
        if scope.is_empty() {
            return Err("codex bridge network search response requires a scope".to_string());
        }
        let search_url = search_url.trim();
        if !is_http_url(search_url) {
            return Err(
                "codex bridge network search response requires an HTTP(S) search URL".to_string(),
            );
        }
        if items.is_empty() {
            return Err(
                "codex bridge network search response requires at least one source link"
                    .to_string(),
            );
        }
        for item in &items {
            if !is_http_url(&item.url) {
                return Err(
                    "codex bridge network search response item requires an HTTP(S) source URL"
                        .to_string(),
                );
            }
        }

        Ok(Self {
            contract_version: CODEX_BRIDGE_CONTRACT_VERSION.to_string(),
            capability: CodexBridgeCapability::NetworkSearch,
            provider: provider.to_string(),
            query: query.to_string(),
            scope: scope.to_string(),
            search_url: search_url.to_string(),
            items,
        })
    }
}

fn is_structured_control_action(action: &str) -> bool {
    const PREFIXES: [&str; 6] = ["click:", "move:", "type:", "press:", "hotkey:", "scroll:"];
    PREFIXES.iter().any(|prefix| action.starts_with(prefix))
}

fn is_http_url(value: &str) -> bool {
    reqwest::Url::parse(value)
        .map(|url| matches!(url.scheme(), "http" | "https"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use crate::kernel::models::LargeModelProvider;

    use super::{
        default_codex_bridge_health_request, CodexBridgeCapability, CodexBridgeCapabilityStatus,
        CodexBridgeControlRequest, CodexBridgeControlResponse, CodexBridgeHealthResponse,
        CodexBridgeNetworkSearchItem, CodexBridgeNetworkSearchRequest,
        CodexBridgeNetworkSearchResponse, CodexBridgeScreenshotRequest,
        CodexBridgeScreenshotResponse, CODEX_BRIDGE_CONTRACT_VERSION,
    };

    #[test]
    fn health_request_serializes_contract_version_and_requested_capabilities() {
        let request = default_codex_bridge_health_request();
        let value = serde_json::to_value(&request).expect("health request serializes");

        assert_eq!(value["contract_version"], CODEX_BRIDGE_CONTRACT_VERSION);
        assert_eq!(
            value["requested_capabilities"],
            serde_json::json!(["computer_screenshot", "computer_control", "network_search"])
        );
    }

    #[test]
    fn network_search_request_serializes_provider_query_and_scope() {
        let request = CodexBridgeNetworkSearchRequest::new(
            LargeModelProvider::ChatGpt,
            "hotel ADR",
            "public web",
        )
        .expect("valid network search request");
        let value = serde_json::to_value(&request).expect("request serializes");

        assert_eq!(value["contract_version"], CODEX_BRIDGE_CONTRACT_VERSION);
        assert_eq!(value["capability"], "network_search");
        assert_eq!(value["large_model_provider"], "chatgpt");
        assert_eq!(value["query"], "hotel ADR");
        assert_eq!(value["scope"], "public web");
    }

    #[test]
    fn network_search_response_requires_source_links() {
        assert!(CodexBridgeNetworkSearchResponse::new(
            "bridge search",
            "hotel ADR",
            "public web",
            "https://bridge.local/search?q=hotel",
            Vec::new(),
        )
        .is_err());
        assert!(CodexBridgeNetworkSearchResponse::new(
            "bridge search",
            "hotel ADR",
            "public web",
            "not a url",
            vec![CodexBridgeNetworkSearchItem {
                title: "Result".to_string(),
                url: "https://example.com/source".to_string(),
                snippet: "Source-backed result.".to_string(),
            }],
        )
        .is_err());

        let response = CodexBridgeNetworkSearchResponse::new(
            "bridge search",
            "hotel ADR",
            "public web",
            "https://bridge.local/search?q=hotel",
            vec![CodexBridgeNetworkSearchItem {
                title: "Result".to_string(),
                url: "https://example.com/source".to_string(),
                snippet: "Source-backed result.".to_string(),
            }],
        )
        .expect("valid response");

        assert_eq!(response.capability, CodexBridgeCapability::NetworkSearch);
        assert_eq!(response.items[0].url, "https://example.com/source");
    }

    #[test]
    fn control_request_requires_target_and_structured_action() {
        assert!(CodexBridgeControlRequest::new("", "click:120,340").is_err());
        assert!(CodexBridgeControlRequest::new("browser", "Click the submit button").is_err());

        let request = CodexBridgeControlRequest::new("browser", "click:120,340")
            .expect("structured control action is valid");

        assert_eq!(request.contract_version, CODEX_BRIDGE_CONTRACT_VERSION);
        assert_eq!(request.capability, CodexBridgeCapability::ComputerControl);
        assert_eq!(request.target, "browser");
        assert_eq!(request.action, "click:120,340");
    }

    #[test]
    fn response_contracts_serialize_without_transport_specific_fields() {
        let health = CodexBridgeHealthResponse {
            contract_version: CODEX_BRIDGE_CONTRACT_VERSION.to_string(),
            runtime_name: "fake-codex-bridge".to_string(),
            runtime_version: "0.1.0".to_string(),
            capabilities: vec![CodexBridgeCapabilityStatus {
                capability: CodexBridgeCapability::ComputerScreenshot,
                available: true,
                note: "ready".to_string(),
            }],
        };
        let screenshot_request = CodexBridgeScreenshotRequest::new(Some("primary".to_string()));
        let control_response =
            CodexBridgeControlResponse::new("clicked left at (120, 340)").expect("valid summary");

        let health_json = serde_json::to_value(&health).expect("health response serializes");
        let screenshot_request_json =
            serde_json::to_value(&screenshot_request).expect("screenshot request serializes");
        let control_response_json =
            serde_json::to_value(&control_response).expect("control response serializes");

        assert_eq!(
            health_json["capabilities"][0]["capability"],
            "computer_screenshot"
        );
        assert_eq!(screenshot_request_json["display_hint"], "primary");
        assert_eq!(control_response_json["capability"], "computer_control");
        assert!(control_response_json.get("endpoint").is_none());
        assert!(control_response_json.get("transport").is_none());
    }

    #[test]
    fn screenshot_response_requires_png_and_dimensions_without_local_paths() {
        let captured_at = Utc.with_ymd_and_hms(2026, 6, 29, 12, 0, 0).unwrap();
        assert!(CodexBridgeScreenshotResponse::new(
            "Primary",
            0,
            1080,
            "iVBORw0KGgo=",
            captured_at
        )
        .is_err());
        assert!(
            CodexBridgeScreenshotResponse::new("Primary", 1920, 1080, "", captured_at).is_err()
        );

        let response =
            CodexBridgeScreenshotResponse::new("Primary", 1920, 1080, "iVBORw0KGgo=", captured_at)
                .expect("valid screenshot response");
        let json = serde_json::to_string(&response).expect("screenshot response serializes");

        assert!(json.contains("\"png_base64\""));
        assert!(!json.contains("D:\\"));
        assert!(!json.contains("computer-screenshots"));
    }
}
