use base64::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::{Client, Response, StatusCode};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::Mutex as AsyncMutex;

pub struct AuthClientProvider {
    client: Client,
    current: Mutex<Option<AuthClient>>,
}

impl AuthClientProvider {
    pub fn new() -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: Client::builder()
                .connect_timeout(Duration::from_secs(10))
                .timeout(Duration::from_secs(60))
                .build()?,
            current: Mutex::new(None),
        })
    }

    pub fn get(&self, url: String, access: String, secret: String) -> AuthClient {
        let url = url.trim_end_matches('/').to_string();
        let mut current = self.current.lock().unwrap();

        if let Some(existing) = current.as_ref() {
            if existing.url == url && existing.access == access && existing.secret == secret {
                return existing.clone();
            }
        }

        let client = AuthClient {
            url,
            access,
            secret,
            client: self.client.clone(),
            token: Arc::new(AsyncMutex::new(None)),
        };

        *current = Some(client.clone());
        client
    }
}

fn create_auth_headers(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();

    if let Ok(value) = HeaderValue::from_str(&format!("Bearer {}", token)) {
        headers.insert(AUTHORIZATION, value);
    }

    headers
}

#[derive(Clone)]
pub struct AuthClient {
    url: String,
    access: String,
    secret: String,
    client: Client,
    token: Arc<AsyncMutex<Option<Arc<String>>>>,
}

impl AuthClient {
    async fn get_or_refresh_token(
        &self,
        rejected: Option<&Arc<String>>,
    ) -> Result<Arc<String>, String> {
        let mut cached = self.token.lock().await;
        if let Some(token) = cached.as_ref() {
            if rejected.map(|old| !Arc::ptr_eq(old, token)).unwrap_or(true) {
                return Ok(token.clone());
            }
        }

        *cached = None;

        let credentials = BASE64_STANDARD.encode(format!("{}:{}", self.access, self.secret));

        let response = self
            .client
            .get(format!("{}/auth", self.url))
            .header(AUTHORIZATION, format!("Basic {}", credentials))
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !response.status().is_success() {
            return Err(format!("Authentication failed: {}", response.status()));
        }

        let token = Arc::new(response.json::<String>().await.map_err(|e| e.to_string())?);
        *cached = Some(token.clone());

        Ok(token)
    }

    async fn request_with_retry<F, Fut>(&self, make_request: F) -> Result<Response, String>
    where
        F: Fn(Client, HeaderMap) -> Fut,
        Fut: std::future::Future<Output = Result<Response, reqwest::Error>>,
    {
        let token = self.get_or_refresh_token(None).await?;
        let response = make_request(self.client.clone(), create_auth_headers(&token))
            .await
            .map_err(|e| e.to_string())?;

        if response.status() == StatusCode::UNAUTHORIZED {
            drop(response);

            let new_token = self.get_or_refresh_token(Some(&token)).await?;

            make_request(self.client.clone(), create_auth_headers(&new_token))
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
