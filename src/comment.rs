use log::{info, warn};
use rand::prelude::IndexedRandom;
use rand::thread_rng;
use reqwest::multipart::Form;
use serde::Deserialize;

use crate::config::Config;
use crate::utils::Utils;

pub struct Comment;

#[derive(Deserialize)]
struct Extra{
    content:String
}

#[derive(Deserialize)]
struct ModeInfo{
    extra:String
}
#[derive(Deserialize)]
struct Data{
    mode_info: ModeInfo,
}
#[derive(Deserialize)]
struct CommentData{
    code:i32,
    data:Data,
    message:Option<String>
}

impl Comment {
    pub async fn new(&self){
        let url = "https://api.live.bilibili.com/msg/send";
        let utils = Utils::new(url).await;
        let config = Config::new().await;
        let comment_msg: Vec<String> = config.msg;
        let rand_msg = comment_msg.choose(&mut thread_rng()).unwrap().clone();
        let form = Form::new()
            .text("bubble", config.bubble.to_string())
            .text("color", config.color.to_string())
            .text("msg", rand_msg)
            .text("fontsize", config.fontsize.to_string())
            .text("jumpfrom", config.jumpfrom.to_string())
            .text("replay_dmid", if let Some(replay_dmid) = config.replay_dmid{replay_dmid} else { String::new() })
            .text("csrf", config.csrf.clone())
            .text("csrf_token", config.csrf)
            .text("statistics", serde_json::to_string(&config.statistics).unwrap())
            .text("mode", config.mode)
            .text("reply_attr", config.reply_attr.to_string())
            .text("rnd", config.rnd.to_string())
            .text("room_type", config.room_type.to_string())
            .text("roomid", config.room_id.to_string())
            .text("reply_mid", config.reply_mid.to_string());
        let result = utils.send_post(form).await;
        let result: CommentData = serde_json::from_str(&*result.text().await.unwrap()).unwrap();
        if  result.code == 0 || result.message.is_none(){
            let content: Extra = serde_json::from_str(&*result.data.mode_info.extra).unwrap();
            info!("已发送弹幕 {}", content.content);
        }else {
            warn!("发送失败 {}", result.message.unwrap())
        }
    }
}