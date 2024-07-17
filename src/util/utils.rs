use reqwest::header::{HeaderMap, COOKIE, USER_AGENT};
use reqwest::multipart::Form;
use reqwest::{Client, Response};
use serde::Serialize;

use crate::logged::load_cookies::CookiesConfig;
use crate::util::error::BilCoreResult;

pub struct Utils {
    url: String,
    client: Client,
}

impl Utils {
    pub async fn new(url: &str) -> Self {
        let load_cookies = CookiesConfig::default();
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".parse().unwrap());
        headers.insert(COOKIE, load_cookies.cookies.parse().unwrap());
        let client = Client::builder().default_headers(headers).build().unwrap();
        Self {
            url: url.to_string(),
            client,
        }
    }

    pub async fn send_post(&self, form: Form) -> BilCoreResult<Response> {
        Ok(self.client.post(&self.url).multipart(form).send().await?)
    }

    pub async fn sne_get<T>(&self, params: Vec<(T, T)>) -> BilCoreResult<Response>
    where
        T: Serialize,
    {
        Ok(self.client.get(&self.url).query(&params).send().await?)
    }

    pub async fn post_with_form<T>(
        &self,
        params: Vec<(T, T)>,
        headers: HeaderMap,
    ) -> BilCoreResult<Response>
    where
        T: Serialize,
    {
        Ok(self
            .client
            .post(&self.url)
            .headers(headers)
            .form(&params)
            .send()
            .await?)
    }

    pub async fn send_get(&self) -> BilCoreResult<Response> {
        Ok(self.client.get(&self.url).send().await?)
    }
}
