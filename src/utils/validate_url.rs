use lazy_static::lazy_static;
use regex::Regex;
use url::Url;

lazy_static! {
    static ref DOMAIN_REGEX: Regex =
        Regex::new(r"^[a-zA-Z0-9-\.]+\.([a-zA-Z]{2,}|[a-zA-Z]{2,}\.[a-zA-Z]{2,})$")
            .expect("Invalid domain regex pattern");
}

pub fn is_valid_domain(url: &str) -> bool {
    let parsed_url = match Url::parse(url) {
        Ok(url) => url,
        Err(_) => return false,
    };

    let host = match parsed_url.host() {
        Some(host) => host.to_string(),
        None => return false,
    };

    DOMAIN_REGEX.is_match(&host)
}

pub fn normalize_url(url: &str) -> Result<String, String> {
    let mut parsed = Url::parse(url)
        .map_err(|e| format!("Failed to parse URL: {}", e))?;

    if parsed.scheme().is_empty() {
        parsed.set_scheme("https")
            .map_err(|_| "Invalid scheme".to_string())?;
    }

    let mut normalized = parsed.to_string();

    if normalized.ends_with('/') && normalized.len() > parsed.scheme().len() + 3 {
        normalized.pop();
    }

    Ok(normalized)
}
