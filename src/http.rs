use crate::error::ApexError;
use reqwest::StatusCode;

const BASE_URL: &'static str = "api.mozambiquehe.re";
const PROTO: &'static str = "http://";

pub struct Http {
    base: url::Url,
    apikey: Option<String>,
}

impl Http {
    pub fn new() -> Self {
        // unwrap this, because proto and base are const
        let base = url::Url::parse(&format!("{}{}", PROTO, BASE_URL)).unwrap();
        Http { base, apikey: None }
    }
    pub fn new_with_auth(key: &str) -> Self {
        let mut http = Http::new();
        http.apikey = Some(key.to_string());
        http
    }

    pub async fn request(&self, path: &str, query: &[(&str, &str)]) -> Result<String, ApexError> {
        let mut url = self.base.join(path)?;
        if let Some(key) = &self.apikey {
            url.query_pairs_mut().append_pair("auth", key);
        }
        query.iter().for_each(|(k, v)| {
            url.query_pairs_mut().append_pair(k, v);
        });
        let body = reqwest::get(url.as_str()).await?;

        let status = body.status();

        match status {
            StatusCode::OK => Ok(body.text().await?),
            StatusCode::BAD_REQUEST => Err(ApexError::APITimeout),
            StatusCode::FORBIDDEN => Err(ApexError::InvalidAPIKey),
            StatusCode::TOO_MANY_REQUESTS => Err(ApexError::RateLimited),
            _ => Err(ApexError::Unknown),
        }
    }
}
