use base64::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, Response, StatusCode};
use std::sync::Mutex;

static TOKEN_CACHE: Mutex<Option<String>> = Mutex::new(None);

pub async fn get_token(url: &str, access: &str, secret: &str) -> Result<String, String> {
    // Check cache first
    if let Ok(cache) = TOKEN_CACHE.lock() {
        if let Some(token) = cache.as_ref() {
            return Ok(token.clone());
        }
    }

    fetch_new_token(url, access, secret).await
}

async fn fetch_new_token(url: &str, access: &str, secret: &str) -> Result<String, String> {
    let credentials = format!("{}:{}", access, secret);
    let encoded = BASE64_STANDARD.encode(credentials.as_bytes());

    let client = Client::new();
    let response = client
        .get(format!("{}/auth", url))
        .header(AUTHORIZATION, format!("Basic {}", encoded))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let token: String = response.json().await.map_err(|e| e.to_string())?;

        // Cache the token
        if let Ok(mut cache) = TOKEN_CACHE.lock() {
            *cache = Some(token.clone());
        }

        Ok(token)
    } else {
        Err(format!("Authentication failed: {}", response.status()))
    }
}

pub fn clear_token_cache() {
    if let Ok(mut cache) = TOKEN_CACHE.lock() {
        *cache = None;
    }
}

pub fn create_auth_headers(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    if let Ok(value) = HeaderValue::from_str(&format!("Bearer {}", token)) {
        headers.insert(AUTHORIZATION, value);
    }
    headers
}

/// Wrapper for authenticated requests with automatic token refresh on 401
pub struct AuthClient {
    url: String,
    access: String,
    secret: String,
    client: Client,
}

impl AuthClient {
    pub fn new(url: String, access: String, secret: String) -> Self {
        Self {
            url,
            access,
            secret,
            client: Client::new(),
        }
    }

    async fn get_or_refresh_token(&self, force_refresh: bool) -> Result<String, String> {
        if force_refresh {
            clear_token_cache();
        }
        get_token(&self.url, &self.access, &self.secret).await
    }

    async fn request_with_retry<F, Fut>(&self, make_request: F) -> Result<Response, String>
    where
        F: Fn(Client, HeaderMap) -> Fut,
        Fut: std::future::Future<Output = Result<Response, reqwest::Error>>,
    {
        // First attempt with cached or new token
        let token = self.get_or_refresh_token(false).await?;
        let headers = create_auth_headers(&token);
        let response = make_request(self.client.clone(), headers)
            .await
            .map_err(|e| e.to_string())?;

        // If unauthorized, refresh token and retry once
        if response.status() == StatusCode::UNAUTHORIZED {
            let new_token = self.get_or_refresh_token(true).await?;
            let new_headers = create_auth_headers(&new_token);
            make_request(self.client.clone(), new_headers)
                .await
                .map_err(|e| e.to_string())
        } else {
            Ok(response)
        }
    }

    pub async fn get(&self, path: &str) -> Result<Response, String> {
        let url = format!("{}{}", self.url, path);
        self.request_with_retry(|client, headers| {
            let url = url.clone();
            async move { client.get(&url).headers(headers).send().await }
        })
        .await
    }

    pub async fn get_with_json<T: serde::Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<Response, String> {
        let url = format!("{}{}", self.url, path);
        self.request_with_retry(|client, headers| {
            let url = url.clone();
            let body_clone = serde_json::to_value(body).unwrap();
            async move {
                client
                    .get(&url)
                    .headers(headers)
                    .json(&body_clone)
                    .send()
                    .await
            }
        })
        .await
    }

    pub async fn post<T: serde::Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<Response, String> {
        let url = format!("{}{}", self.url, path);
        self.request_with_retry(|client, headers| {
            let url = url.clone();
            let body_clone = serde_json::to_value(body).unwrap();
            async move {
                client
                    .post(&url)
                    .headers(headers)
                    .json(&body_clone)
                    .send()
                    .await
            }
        })
        .await
    }

    pub async fn patch<T: serde::Serialize>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<Response, String> {
        let url = format!("{}{}", self.url, path);
        self.request_with_retry(|client, headers| {
            let url = url.clone();
            let body_clone = serde_json::to_value(body).unwrap();
            async move {
                client
                    .patch(&url)
                    .headers(headers)
                    .json(&body_clone)
                    .send()
                    .await
            }
        })
        .await
    }

    pub async fn delete(&self, path: &str) -> Result<Response, String> {
        let url = format!("{}{}", self.url, path);
        self.request_with_retry(|client, headers| {
            let url = url.clone();
            async move { client.delete(&url).headers(headers).send().await }
        })
        .await
    }
}
