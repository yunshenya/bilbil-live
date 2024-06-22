use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CookiesConfig {
    pub refresh_token: String,
    pub cookies: String,
    pub is_login: bool
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
