use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs};

use reqwest::blocking::{Client, Response};
use reqwest::header::LOCATION;
use reqwest::Url;

pub fn validate_public_http_url_syntax(value: &str) -> Result<Url, String> {
    let value = value.trim();
    let url = Url::parse(value).map_err(|error| format!("network URL is invalid: {error}"))?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err("network URL must use http:// or https://".to_string());
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err("network URL credentials are not allowed".to_string());
    }
    let host = url
        .host_str()
        .ok_or_else(|| "network URL host is required".to_string())?;
    if let Ok(address) = host.parse::<IpAddr>() {
        ensure_public_ip(address)?;
    } else {
        ensure_public_domain(host)?;
    }
    Ok(url)
}

pub fn validate_public_http_url_for_connection(value: &str) -> Result<Url, String> {
    let url = validate_public_http_url_syntax(value)?;
    let host = url
        .host_str()
        .ok_or_else(|| "network URL host is required".to_string())?;
    let port = url
        .port_or_known_default()
        .ok_or_else(|| "network URL port is unavailable".to_string())?;
    let addresses = (host, port)
        .to_socket_addrs()
        .map_err(|error| format!("network host could not be resolved safely: {error}"))?
        .collect::<Vec<_>>();
    if addresses.is_empty() {
        return Err("network host resolved to no addresses".to_string());
    }
    for address in addresses {
        ensure_public_ip(address.ip())?;
    }
    Ok(url)
}

pub fn ensure_public_remote_addr(address: Option<SocketAddr>) -> Result<(), String> {
    let address = address
        .ok_or_else(|| "network response did not expose a verifiable remote address".to_string())?;
    ensure_public_ip(address.ip())
}

pub fn send_public_get(
    client: &Client,
    initial_url: &str,
    redirect_limit: usize,
) -> Result<Response, String> {
    let mut url = validate_public_http_url_for_connection(initial_url)?;
    for redirect_count in 0..=redirect_limit {
        let response = client
            .get(url.clone())
            .send()
            .map_err(|error| format!("public network request failed: {error}"))?;
        ensure_public_remote_addr(response.remote_addr())?;
        if !response.status().is_redirection() {
            return Ok(response);
        }
        if redirect_count >= redirect_limit {
            return Err(format!(
                "network redirect limit of {redirect_limit} was exceeded"
            ));
        }
        let location = response
            .headers()
            .get(LOCATION)
            .ok_or_else(|| "network redirect did not include a Location header".to_string())?
            .to_str()
            .map_err(|_| "network redirect Location header is not valid text".to_string())?;
        let next_url = url
            .join(location)
            .map_err(|error| format!("network redirect URL is invalid: {error}"))?;
        url = validate_public_http_url_for_connection(next_url.as_str())?;
    }
    Err("network redirect processing ended unexpectedly".to_string())
}

pub fn ensure_public_ip(address: IpAddr) -> Result<(), String> {
    let public = match address {
        IpAddr::V4(address) => ipv4_is_public(address),
        IpAddr::V6(address) => ipv6_is_public(address),
    };
    if public {
        Ok(())
    } else {
        Err(format!(
            "DS Agent network sandbox blocked non-public address `{address}`"
        ))
    }
}

fn ensure_public_domain(domain: &str) -> Result<(), String> {
    let domain = domain.trim_end_matches('.').to_ascii_lowercase();
    let blocked_suffixes = [
        "localhost",
        ".localhost",
        ".local",
        ".internal",
        ".lan",
        ".home.arpa",
    ];
    if domain.is_empty()
        || !domain.contains('.')
        || blocked_suffixes
            .iter()
            .any(|suffix| domain == *suffix || domain.ends_with(suffix))
        || matches!(
            domain.as_str(),
            "metadata.google.internal" | "metadata.azure.internal" | "instance-data"
        )
    {
        return Err(format!(
            "DS Agent network sandbox blocked local or private host `{domain}`"
        ));
    }
    Ok(())
}

fn ipv4_is_public(address: Ipv4Addr) -> bool {
    let [a, b, c, d] = address.octets();
    if a == 0
        || a == 10
        || a == 127
        || a >= 224
        || (a == 100 && (64..=127).contains(&b))
        || (a == 169 && b == 254)
        || (a == 172 && (16..=31).contains(&b))
        || (a == 192 && b == 168)
        || (a == 198 && (18..=19).contains(&b))
        || (a == 192 && b == 0 && c == 0)
        || (a == 192 && b == 0 && c == 2)
        || (a == 198 && b == 51 && c == 100)
        || (a == 203 && b == 0 && c == 113)
        || (a == 192 && b == 88 && c == 99)
        || (a == 255 && b == 255 && c == 255 && d == 255)
    {
        return false;
    }
    true
}

fn ipv6_is_public(address: Ipv6Addr) -> bool {
    if address.is_unspecified() || address.is_loopback() || address.is_multicast() {
        return false;
    }
    if let Some(mapped) = address.to_ipv4_mapped() {
        return ipv4_is_public(mapped);
    }
    let segments = address.segments();
    if (segments[0] & 0xfe00) == 0xfc00
        || (segments[0] & 0xffc0) == 0xfe80
        || (segments[0] & 0xffc0) == 0xfec0
        || (segments[0] == 0x2001 && segments[1] == 0x0db8)
        || (segments[0] == 0x2001 && segments[1] == 0x0002)
        || (segments[0] == 0x0100 && segments[1..4] == [0, 0, 0])
    {
        return false;
    }
    (segments[0] & 0xe000) == 0x2000
}

#[cfg(test)]
mod tests {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    use super::{ensure_public_ip, ensure_public_remote_addr, validate_public_http_url_syntax};

    #[test]
    fn public_url_policy_rejects_local_private_metadata_and_credential_targets() {
        for url in [
            "http://127.0.0.1/admin",
            "http://[::1]/admin",
            "http://10.0.0.1/private",
            "http://172.16.0.1/private",
            "http://192.168.1.1/private",
            "http://169.254.169.254/latest/meta-data",
            "http://localhost/admin",
            "http://printer.local/status",
            "http://user:password@example.com/private",
            "file:///C:/Windows/System32/drivers/etc/hosts",
        ] {
            assert!(
                validate_public_http_url_syntax(url).is_err(),
                "{url} must be blocked"
            );
        }
    }

    #[test]
    fn public_url_policy_accepts_public_http_and_https_targets() {
        assert!(validate_public_http_url_syntax("https://example.com/docs?q=agent").is_ok());
        assert!(validate_public_http_url_syntax("http://93.184.216.34/").is_ok());
    }

    #[test]
    fn public_ip_policy_rejects_non_global_ranges() {
        for ip in [
            IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            IpAddr::V4(Ipv4Addr::new(100, 64, 0, 1)),
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            IpAddr::V4(Ipv4Addr::new(169, 254, 1, 1)),
            IpAddr::V4(Ipv4Addr::new(198, 18, 0, 1)),
            IpAddr::V6(Ipv6Addr::LOCALHOST),
            IpAddr::V6("fc00::1".parse().expect("unique local ipv6")),
            IpAddr::V6("fe80::1".parse().expect("link local ipv6")),
            IpAddr::V6("2001:db8::1".parse().expect("documentation ipv6")),
        ] {
            assert!(ensure_public_ip(ip).is_err(), "{ip} must be blocked");
        }
        assert!(ensure_public_ip(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))).is_ok());
        assert!(ensure_public_ip("2606:4700:4700::1111".parse().expect("public ipv6")).is_ok());
    }

    #[test]
    fn public_remote_address_policy_requires_a_verifiable_public_socket() {
        assert!(ensure_public_remote_addr(None).is_err());
        assert!(ensure_public_remote_addr(Some(SocketAddr::from(([127, 0, 0, 1], 443)))).is_err());
        assert!(ensure_public_remote_addr(Some(SocketAddr::from(([8, 8, 8, 8], 443)))).is_ok());
    }
}
