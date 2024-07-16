use crate::arrangement::api::CommentSendUrl;
use crate::arrangement::config::Config;
use crate::logged::load_cookies::CookiesConfig;
use crate::util::error::BilCoreResult;
use crate::util::utils::Utils;
use log::{info, warn};
use rand::prelude::IndexedRandom;
use rand::thread_rng;
use reqwest::multipart::Form;
use serde::Deserialize;
use std::sync::Arc;

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
    pub async fn new() -> Self {
        let utils = Utils::new(CommentSendUrl::get_api()).await;
        let config = Arc::new(Config::new().await);
        Self { utils, config }
    }

    pub async fn build_form(&self, rand_msg: Option<String>) -> BilCoreResult<Form> {
        let csrf = CookiesConfig::csrf();
        let msg = if let Some(msg) = rand_msg {
            msg
        } else {
            let msg_list = &Arc::clone(&self.config).msg;
            msg_list.choose(&mut thread_rng()).unwrap().to_string()
        };
        Ok(Form::new()
            .text("bubble", "0")
            .text("color", self.config.color.to_string())
            .text("msg", msg)
            .text("fontsize", "25")
            .text("jumpfrom", self.config.jumpfrom.to_string())
            .text(
                "replay_dmid",
                Arc::clone(&self.config.replay_dmid).lock().await.take().unwrap_or_default(),
            )
            .text("csrf", csrf.to_string())
            .text("csrf_token", csrf.to_string())
            .text(
                "statistics",
                serde_json::to_string(&self.config.statistics)?,
            )
            .text("mode", self.config.mode.to_string())
            .text("reply_attr", "0")
            .text("rnd", CookiesConfig::rnd().to_string())
            .text("room_type", self.config.room_type.to_string())
            .text("roomid", self.config.room_id.to_string())
            .text("reply_mid", "0"))
    }

    pub async fn send(&self, form: Form) -> BilCoreResult<()> {
        match self.utils.send_post(form).await {
            Ok(response) => {
                let result = response.text().await?;
                let comment_data = serde_json::from_str::<CommentData>(&result)?;
                if comment_data.code == 0 || comment_data.message.is_none() {
                    let content =
                        serde_json::from_str::<Extra>(&comment_data.data.mode_info.extra)?;
                    info!("消息发送成功 {}", content.content);
                    Ok(())
                } else {
                    warn!("消息发送失败 {}", comment_data.message.unwrap_or_default());
                    Ok(())
                }
            }
            Err(err) => {
                warn!("{}", err.to_string());
                Ok(())
            }
        }
    }
}
