use crate::config::Config;
use crate::utils::Utils;
use log::{info, warn};
use reqwest::multipart::Form;
use serde::Deserialize;

pub struct LikeSend;

#[derive(Deserialize)]
struct LikeResult {
    code: i32,
}

impl LikeSend {
    pub async fn new() -> bool {
        let url =
            "https://api.live.bilibili.com/xlive/app-ucenter/v1/like_info_v3/like/likeReportV3"
                .to_string();
        let utils = Utils::new(url).await;
        let config = Config::new().await;
        let form = Form::new()
            .text("click_time", config.click_time.to_string())
            .text("room_id", config.room_id.to_string())
            .text("uid", config.uid.to_string())
            .text("anchor_id", config.anchor_id.to_string())
            .text("csrf_token", config.csrf.clone())
            .text("csrf", config.csrf)
            .text(
                "visit_id",
                if let Some(visit_id) = config.visit_id {
                    visit_id
                } else {
                    String::new()
                },
            );

        let result = utils.send_post(form).await;
        let data_code: LikeResult = serde_json::from_str(&*result.text().await.unwrap()).unwrap();
        if data_code.code == 0 {
            info!("已点赞");
            return true;
        } else {
            warn!("点赞失败");
            false
        }
    }
}
