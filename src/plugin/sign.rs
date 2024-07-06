use log::{info, warn};
use reqwest::header::{HeaderMap, REFERER};
use serde::{Deserialize};
use crate::api::SINGN;
use crate::load_cookies::CookiesConfig;
use crate::utils::Utils;

#[derive(Deserialize)]
struct SignJson{
    message:String
}

pub async fn sign(){
    let mut headers = HeaderMap::new();
    let csrf = &*CookiesConfig::csrf();
    headers.insert(REFERER, "https://www.bilibili.com".parse().unwrap());
    let params = vec![("csrf", csrf)];
    let utils = Utils::new(SINGN).await;
    match utils.post_with_form(params, headers).await {
        Ok(resp) => {
            let json = serde_json::from_str::<SignJson>(&resp.text().await.unwrap()).unwrap();
            info!("今日签到: {}", json.message)
        }
        Err(_) => {
            warn!("请求发送失败")
        }
    }
}