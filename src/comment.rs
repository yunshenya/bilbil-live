use crate::api::{COMMENT_SEND_URL, ONE_MSG};
use log::{info, warn};
use rand::prelude::IndexedRandom;
use rand::thread_rng;
use reqwest::multipart::Form;
use reqwest::Client;
use serde::Deserialize;

use crate::config::Config;
use crate::utils::Utils;

#[derive(Default)]
pub struct Comment {
    utils: Utils,
    config: Config,
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
        let config = Config::new().await;
        Self { utils, config }
    }

    pub async fn build_form(&self, rand_msg: Option<String>) -> Form {
        let msg = if let Some(msg) = rand_msg {
            msg
        } else {
            let msg_list = self.config.msg.clone();
            let _ = msg_list.choose(&mut thread_rng()).unwrap().clone();
            let string = self.get_msg().await;
            self.decode_unicode(&*string)
                .replace("\"", "")
                .replace("]", "")
        };
        Form::new()
            .text("bubble", self.config.bubble.to_string())
            .text("color", self.config.color.to_string())
            .text("msg", msg)
            .text("fontsize", self.config.fontsize.to_string())
            .text("jumpfrom", self.config.jumpfrom.to_string())
            .text(
                "replay_dmid",
                if let Some(replay_dmid) = self.config.replay_dmid.clone() {
                    replay_dmid
                } else {
                    String::new()
                },
            )
            .text("csrf", self.config.csrf.clone())
            .text("csrf_token", self.config.csrf.clone())
            .text(
                "statistics",
                serde_json::to_string(&self.config.statistics).unwrap(),
            )
            .text("mode", self.config.mode.clone())
            .text("reply_attr", self.config.reply_attr.to_string())
            .text("rnd", self.config.rnd.to_string())
            .text("room_type", self.config.room_type.to_string())
            .text("roomid", self.config.room_id.to_string())
            .text("reply_mid", self.config.reply_mid.to_string())
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

    pub async fn get_msg(&self) -> String {
        let client = Client::new();
        let response = client
            .get(ONE_MSG)
            .send()
            .await
            .unwrap();
        response.text().await.unwrap()
    }

    fn decode_unicode(&self, unicode_str: &str) -> String {
        let mut decoded_str = String::new();
        let parts: Vec<&str> = unicode_str.split("\\u").collect();
        for part in parts.iter().skip(1) {
            if let Ok(hex) = u16::from_str_radix(&part[0..4], 16) {
                if let Some(ch) = char::from_u32(hex as u32) {
                    decoded_str.push(ch);
                }
            }
            if part.len() > 4 {
                decoded_str.push_str(&part[4..]);
            }
        }
        decoded_str
    }
}
