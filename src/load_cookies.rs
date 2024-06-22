use std::borrow::Cow;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CookiesConfig {
    pub refresh_token: String,
    pub cookies: String,
    pub is_login: bool,
}


impl CookiesConfig{
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
}

impl Default for CookiesConfig {
    fn default() -> Self {
        let cookies_str = include_str!("../config/cookies.yaml");
        let cookies: CookiesConfig = serde_yaml::from_str(cookies_str).unwrap();
        Self {
            refresh_token: cookies.refresh_token,
            cookies: cookies.cookies,
            is_login: cookies.is_login
        }
    }
}
