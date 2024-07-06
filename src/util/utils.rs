use crate::logged::load_cookies::CookiesConfig;
use log::error;
use reqwest::header::{HeaderMap, COOKIE, USER_AGENT};
use reqwest::multipart::Form;
use reqwest::{Client, ClientBuilder, Response};

#[derive(Default)]
pub struct Utils {
    url: String,
    headers: HeaderMap,
}

impl Utils {
    pub async fn new(url: &str) -> Self {
        let load_cookies = CookiesConfig::default();
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36".parse().unwrap());
        headers.insert(COOKIE, load_cookies.cookies.parse().unwrap());
        Self {
            url: url.to_string(),
            headers,
        }
    }

    pub async fn send_post(&self, form: Form) -> Response {
        let client = Client::builder()
            .default_headers(self.headers.clone())
            .build()
            .unwrap();
        client.post(&self.url).multipart(form).send().await.unwrap()
    }

    pub async fn sne_get(&self, params: Vec<(&str, &str)>) -> Response {
        let client = Client::builder()
            .default_headers(self.headers.clone())
            .build()
            .unwrap();
        client.get(&self.url).query(&params).send().await.unwrap()
    }

    pub async fn post_with_form(
        &self,
        params: Vec<(&str, &str)>,
        headers: HeaderMap,
    ) -> Result<Response, ()> {
        let client = ClientBuilder::new()
            .default_headers(self.headers.clone())
            .build()
            .unwrap();
        match client
            .post(&self.url)
            .headers(headers)
            .form(&params)
            .send()
            .await
        {
            Ok(res) => Ok(res),
            Err(err) => Err(error!("{}", err)),
        }
    }
}
