use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Loader {
    room: RoomInfo
}

#[derive(Deserialize)]
struct RoomInfo {
    room_id: i128,
    anchor_id: i128,
    bubble: i32,
    msg: Vec<String>,
    color: i32,
    mode: String,
    room_type: i32,
    jumpfrom: i32,
    reply_mid: i32,
    reply_attr: i32,
    replay_dmid: Option<String>,
    statistics: Statistics,
    fontsize: i32,
    rnd: i64,
    click_time: i32,
    visit_id: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Statistics {
    #[serde(rename = "appId")]
    pub app_id: i32,
    pub platform: i32,
}

#[derive(Default)]
pub struct Config {
    pub room_id: i128,
    pub anchor_id: i128,
    pub bubble: i32,
    pub msg: Vec<String>,
    pub color: i32,
    pub mode: String,
    pub room_type: i32,
    pub jumpfrom: i32,
    pub reply_mid: i32,
    pub reply_attr: i32,
    pub replay_dmid: Option<String>,
    pub statistics: Statistics,
    pub fontsize: i32,
    pub rnd: i64,
    pub click_time: i32,
    pub visit_id: Option<String>,
}

impl Config {
    pub async fn new() -> Self {
        let yaml_str = include_str!("../config/config.yaml");
        let loader: Loader = serde_yaml::from_str(yaml_str).unwrap();
        Self {
            room_id: loader.room.room_id,
            anchor_id: loader.room.anchor_id,
            bubble: loader.room.bubble,
            msg: loader.room.msg,
            color: loader.room.color,
            rnd: loader.room.rnd,
            room_type: loader.room.room_type,
            jumpfrom: loader.room.jumpfrom,
            reply_mid: loader.room.reply_mid,
            reply_attr: loader.room.reply_attr,
            replay_dmid: loader.room.replay_dmid,
            statistics: loader.room.statistics,
            click_time: loader.room.click_time,
            visit_id: loader.room.visit_id,
            mode: loader.room.mode,
            fontsize: loader.room.fontsize,
        }
    }
}
