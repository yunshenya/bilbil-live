use reqwest::header::{HeaderMap, COOKIE, USER_AGENT};
use reqwest::multipart::Form;
use reqwest::{Client, Error, Response};
use serde::Serialize;

use crate::logged::load_cookies::CookiesConfig;

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

    pub async fn send_post(&self, form: Form) -> Result<Response,Error> {
        match self.client
            .post(&self.url)
            .multipart(form)
            .send()
            .await {
            Ok(resp) => Ok(resp),
            Err(err) => Err(err)
        }
    }

    pub async fn sne_get<T>(&self, params: Vec<(T, T)>) -> Response
    where
        T: Serialize,
    {
        self.client
            .get(&self.url)
            .query(&params)
            .send()
            .await
            .unwrap()
    }

    pub async fn post_with_form<T>(
        &self,
        params: Vec<(T, T)>,
        headers: HeaderMap,
    ) -> Result<Response, String>
    where
        T: Serialize,
    {
        match self
            .client
            .post(&self.url)
            .headers(headers)
            .form(&params)
            .send()
            .await
        {
            Ok(res) => Ok(res),
            Err(err) => Err(err.to_string()),
        }
    }

    pub async fn send_get(&self) -> Result<Response, Error> {
        match self.client.get(&self.url).send().await {
            Ok(resp) => Ok(resp),
            Err(err) => Err(err),
        }
    }
}
