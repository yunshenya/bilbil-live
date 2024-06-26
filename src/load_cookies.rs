use crate::api::{COOKIES_PATH, GET_LIVE_INFO};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fs::read_to_string;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
pub struct CookiesConfig {
    pub refresh_token: String,
    pub cookies: String,
    pub is_login: bool,
    pub uid: u128,
}

#[derive(Serialize, Deserialize)]
pub struct RoomInfo {
    pub data: RoomInfoData,
}

#[derive(Deserialize, Serialize)]
pub struct RoomInfoData {
    pub uid: u128,
    pub room_id: u128,
}

impl CookiesConfig {
    pub fn csrf() -> Cow<'static, str> {
        let config = CookiesConfig::default();
        for str in config.cookies.split("; ") {
            let mut parts = str.split("=");
            if parts.next().unwrap_or("") == "bili_jct" {
                if let Some(csrf) = parts.next() {
                    // 使用Cow来灵活处理是复制还是借用
                    return Cow::Owned(csrf.to_string());
                }
            }
        }
        // 如果找不到bili_jct，则返回一个空字符串，这里使用了Cow的Owned变体
        Cow::Owned(String::new())
    }

    pub async fn anchor_id(room_id: u128) -> RoomInfo {
        let client = Client::new();
        let params = [("room_id", room_id)];
        let room_info_resp = client
            .get(GET_LIVE_INFO)
            .query(&params)
            .send()
            .await
            .unwrap();
        let room_info: RoomInfo =
            serde_json::from_str(&*room_info_resp.text().await.unwrap()).unwrap();
        return room_info;
    }

    pub fn rnd() -> u64 {
        let current_time = SystemTime::now();
        let since_epoch = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_epoch.as_secs()
    }
}

impl Default for CookiesConfig {
    fn default() -> Self {
        let cookies_str = read_to_string(COOKIES_PATH).unwrap();
        let cookies: CookiesConfig = serde_yaml::from_str(&*cookies_str).unwrap();
        Self {
            refresh_token: cookies.refresh_token,
            cookies: cookies.cookies,
            is_login: cookies.is_login,
            uid: cookies.uid,
        }
    }
}
