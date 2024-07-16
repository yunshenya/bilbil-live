use reqwest::multipart::Form;
use serde::Deserialize;

use crate::arrangement::api::SendLikeUrl;
use crate::arrangement::config::Config;
use crate::logged::load_cookies::CookiesConfig;
use crate::util::error::{BilCoreResult, BilError};
use crate::util::utils::Utils;

pub struct LikeSend;

#[derive(Deserialize)]
struct LikeResult {
    code: i32,
}

impl LikeSend {
    pub async fn new() -> BilCoreResult<Self> {
        let utils = Utils::new(SendLikeUrl::get_api()).await;
        let config = Config::new().await;
        let load_config = CookiesConfig::default();
        let csrf = CookiesConfig::csrf();
        let form = Form::new()
            .text("click_time", config.click_time.to_string())
            .text("room_id", config.room_id.to_string())
            .text("uid", load_config.uid.to_string())
            .text(
                "anchor_id",
                CookiesConfig::anchor_id(config.room_id)
                    .await?
                    .data
                    .uid
                    .to_string(),
            )
            .text("csrf_token", csrf.to_string())
            .text("csrf", csrf.to_string())
            .text("visit_id", config.visit_id.unwrap_or_default());

        match utils.send_post(form).await {
            Ok(result) => {
                let data_code = serde_json::from_str::<LikeResult>(&result.text().await?)?;
                if data_code.code == 0 {
                    Ok(Self)
                } else {
                    Err(BilError::Params(String::from("字符串错误")))
                }
            }
            Err(err) => Err(err),
        }
    }
}
