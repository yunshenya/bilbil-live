use crate::arrangement::api::COMMENT_SEND_URL;
use log::{info, warn};
use rand::prelude::IndexedRandom;
use rand::thread_rng;
use reqwest::multipart::Form;
use serde::Deserialize;
use std::borrow::Cow;
use std::sync::Arc;

use crate::arrangement::config::Config;
use crate::logged::load_cookies::CookiesConfig;
use crate::util::utils::Utils;

#[derive(Default)]
pub struct Comment {
    utils: Utils,
    config: Arc<Config>,
}

#[derive(Deserialize)]
struct Extra {
    content: String,
}

#[derive(Deserialize)]
struct ModeInfo {
    extra: String,
}
#[derive(Deserialize)]
struct Data {
    mode_info: ModeInfo,
}
#[derive(Deserialize)]
struct CommentData {
    code: i32,
    data: Data,
    message: Option<String>,
}

impl Comment {
    pub async fn new(&self) -> Self {
        let utils = Utils::new(COMMENT_SEND_URL).await;
        let config = Arc::new(Config::new().await);
        Self { utils, config }
    }

    pub async fn build_form(&self, rand_msg: Option<String>) -> Form {
        let msg = if let Some(msg) = rand_msg {
            msg
        } else {
            let msg_list = &Arc::clone(&self.config).msg;
            msg_list.choose(&mut thread_rng()).unwrap().to_string()
        };
        Form::new()
            .text("bubble", "0")
            .text("color", self.config.color.to_string())
            .text("msg", msg)
            .text("fontsize", "25")
            .text("jumpfrom", self.config.jumpfrom.to_string())
            .text(
                "replay_dmid",
                if let Some(replay_dmid) = self.config.replay_dmid.clone() {
                    replay_dmid
                } else {
                    String::new()
                },
            )
            .text("csrf", CookiesConfig::csrf())
            .text("csrf_token", CookiesConfig::csrf())
            .text(
                "statistics",
                serde_json::to_string(&self.config.statistics).unwrap(),
            )
            .text("mode", Cow::Owned((&self.config.mode).to_string()))
            .text("reply_attr", "0")
            .text("rnd", CookiesConfig::rnd().to_string())
            .text("room_type", self.config.room_type.to_string())
            .text("roomid", self.config.room_id.to_string())
            .text("reply_mid", "0")
    }

    pub async fn send(&self, form: Form) {
        let result = self.utils.send_post(form).await.text().await.unwrap();
        let comment_data: CommentData = serde_json::from_str(&*result).unwrap();
        if comment_data.code == 0 || comment_data.message.is_none() {
            let content: Extra = serde_json::from_str(&*comment_data.data.mode_info.extra).unwrap();
            info!("消息发送成功 {}", content.content)
        } else {
            warn!("消息发送失败 {}", comment_data.message.unwrap())
        }
    }
}
