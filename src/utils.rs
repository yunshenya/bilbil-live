use reqwest::header::{HeaderMap, COOKIE, USER_AGENT};
use reqwest::multipart::Form;
use reqwest::{Client, Response};

use crate::config::Config;

#[derive(Default)]
pub struct Utils {
    url: String,
    headers: HeaderMap,
}

impl Utils {
    pub async fn new(url: String) -> Self {
        let cookies = Config::new().await;
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".parse().unwrap());
        headers.insert(COOKIE, cookies.cookies.parse().unwrap());
        Self { url, headers }
    }

    pub async fn send_post(&self, form: Form) -> Response {
        let client = Client::builder()
            .default_headers(self.headers.clone())
            .build()
            .unwrap();
        client
            .post(self.url.clone())
            .multipart(form)
            .send()
            .await
            .unwrap()
    }

    pub async fn sne_get(&self, params: Vec<(&str, &str)>) -> Response {
        let client = Client::builder()
            .default_headers(self.headers.clone())
            .build()
            .unwrap();
        client
            .get(self.url.clone())
            .query(&params)
            .send()
            .await
            .unwrap()
    }
}
