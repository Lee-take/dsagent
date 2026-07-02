use std::net::IpAddr;
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::kernel::codex_bridge_contract::{
    default_codex_bridge_health_request, CodexBridgeControlRequest, CodexBridgeControlResponse,
    CodexBridgeHealthResponse, CodexBridgeNetworkSearchRequest, CodexBridgeNetworkSearchResponse,
    CodexBridgeScreenshotRequest, CodexBridgeScreenshotResponse,
};

#[derive(Debug)]
pub struct CodexBridgeHttpClient {
    endpoint: String,
    client: reqwest::blocking::Client,
}

impl CodexBridgeHttpClient {
    pub fn new(endpoint: &str, timeout: Duration) -> Result<Self, String> {
        let endpoint = normalize_http_endpoint(endpoint)?;
        let client = reqwest::blocking::Client::builder()
            .user_agent("DeepSeek-Agent-OS/0.1.0 external-bridge")
            .timeout(timeout)
            .build()
            .map_err(|error| format!("local bridge service HTTP client setup failed: {error}"))?;

        Ok(Self { endpoint, client })
    }

    pub fn health(&self) -> Result<CodexBridgeHealthResponse, String> {
        self.post_json("health", &default_codex_bridge_health_request())
    }

    pub fn screenshot(
        &self,
        request: &CodexBridgeScreenshotRequest,
    ) -> Result<CodexBridgeScreenshotResponse, String> {
        self.post_json("screenshot", request)
    }

    pub fn control(
        &self,
        request: &CodexBridgeControlRequest,
    ) -> Result<CodexBridgeControlResponse, String> {
        self.post_json("control", request)
    }

    pub fn network_search(
        &self,
        request: &CodexBridgeNetworkSearchRequest,
    ) -> Result<CodexBridgeNetworkSearchResponse, String> {
        self.post_json("network-search", request)
    }

    fn post_json<Request, Response>(
        &self,
        route: &str,
        request: &Request,
    ) -> Result<Response, String>
    where
        Request: Serialize,
        Response: DeserializeOwned,
    {
        let url = self.route_url(route)?;
        let response = self
            .client
            .post(url)
            .json(request)
            .send()
            .map_err(|error| format!("local bridge service HTTP request failed: {error}"))?;
        let status = response.status();
        let body = response.text().map_err(|error| {
            format!("local bridge service HTTP response could not be read: {error}")
        })?;

        if !status.is_success() {
            return Err(format!(
                "local bridge service HTTP request returned HTTP {}: {}",
                status.as_u16(),
                truncate_for_error(&body, 240)
            ));
        }

        serde_json::from_str(&body).map_err(|error| {
            format!("local bridge service HTTP response could not be parsed: {error}")
        })
    }

    fn route_url(&self, route: &str) -> Result<String, String> {
        let route = route.trim_start_matches('/');
        let url = format!("{}/{route}", self.endpoint);
        reqwest::Url::parse(&url)
            .map(|parsed| parsed.to_string())
            .map_err(|error| format!("local bridge service HTTP route URL is invalid: {error}"))
    }
}

fn normalize_http_endpoint(endpoint: &str) -> Result<String, String> {
    let endpoint = endpoint.trim().trim_end_matches('/');
    if endpoint.is_empty() {
        return Err("local bridge service HTTP endpoint is required".to_string());
    }
    let url = reqwest::Url::parse(endpoint)
        .map_err(|error| format!("local bridge service HTTP endpoint is invalid: {error}"))?;
    match url.scheme() {
        "http" | "https" => {}
        scheme => {
            return Err(format!(
                "local bridge service HTTP endpoint must use http or https, got '{scheme}'"
            ))
        }
    }
    if !is_loopback_url(&url) {
        return Err(
            "local bridge service HTTP endpoint must use a loopback host such as 127.0.1.0 or localhost"
                .to_string(),
        );
    }

    Ok(endpoint.to_string())
}

fn is_loopback_url(url: &reqwest::Url) -> bool {
    let Some(host) = url.host_str() else {
        return false;
    };
    if host.eq_ignore_ascii_case("localhost") {
        return true;
    }
    host.parse::<IpAddr>()
        .map(|address| address.is_loopback())
        .unwrap_or(false)
}

fn truncate_for_error(value: &str, max_chars: usize) -> String {
    value.chars().take(max_chars).collect()
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread::JoinHandle;

    use crate::kernel::codex_bridge_contract::{
        CodexBridgeNetworkSearchRequest, CODEX_BRIDGE_CONTRACT_VERSION,
    };
    use crate::kernel::models::LargeModelProvider;

    use super::CodexBridgeHttpClient;

    #[test]
    fn codex_bridge_http_client_rejects_non_loopback_endpoints() {
        let error = CodexBridgeHttpClient::new(
            "https://example.com",
            std::time::Duration::from_millis(100),
        )
        .expect_err("remote endpoint should be rejected");

        assert!(error.contains("loopback"));
        assert!(error.contains("local bridge service"));
        assert!(!error.contains("external bridge"));
        assert!(!error.contains("codex bridge"));
    }

    #[test]
    fn codex_bridge_http_client_posts_network_search_contract() {
        let response_body = serde_json::json!({
            "contract_version": CODEX_BRIDGE_CONTRACT_VERSION,
            "capability": "network_search",
            "provider": "external bridge search",
            "query": "hotel ADR",
            "scope": "public web",
            "search_url": "https://bridge.local/search?q=hotel",
            "items": [
                {
                    "title": "Source",
                    "url": "https://example.com/source",
                    "snippet": "A source-backed result."
                }
            ]
        })
        .to_string();
        let (endpoint, handle) = serve_one_json_response(response_body);
        let client = CodexBridgeHttpClient::new(&endpoint, std::time::Duration::from_secs(2))
            .expect("http client");
        let request =
            CodexBridgeNetworkSearchRequest::new(LargeModelProvider::ChatGpt, "hotel ADR", "")
                .expect("request");

        let response = client
            .network_search(&request)
            .expect("network search response");
        let recorded = handle.join().expect("server joins");

        assert_eq!(response.items[0].url, "https://example.com/source");
        assert!(recorded.raw.starts_with("POST /network-search "));
        let headers = recorded.raw.split("\r\n\r\n").next().unwrap_or_default();
        let normalized_headers = headers.to_ascii_lowercase();
        assert!(normalized_headers.contains("user-agent: deepseek-agent-os/0.1.0 external-bridge"));
        assert!(!headers.contains("codex-bridge"));
        assert!(recorded.raw.contains("\"capability\":\"network_search\""));
        assert!(recorded
            .raw
            .contains("\"large_model_provider\":\"chatgpt\""));
        assert!(recorded.raw.contains("\"scope\":\"public web\""));
    }

    struct RecordedHttpRequest {
        raw: String,
    }

    fn serve_one_json_response(response_body: String) -> (String, JoinHandle<RecordedHttpRequest>) {
        let listener = TcpListener::bind("127.0.1.0:0").expect("bind fake bridge server");
        let endpoint = format!("http://{}", listener.local_addr().expect("local addr"));
        let handle = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("accept fake bridge request");
            let mut buffer = [0_u8; 4096];
            let bytes_read = stream.read(&mut buffer).expect("read request");
            let raw = String::from_utf8(buffer[..bytes_read].to_vec()).expect("request utf8");
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
}
