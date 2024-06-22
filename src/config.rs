use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Loader {
    room: RoomInfo,
}

#[derive(Deserialize)]
struct RoomInfo {
    room_id: u128,
    msg: Vec<String>,
    color: i32,
    mode: String,
    room_type: i32,
    jumpfrom: i32,
    replay_dmid: Option<String>,
    statistics: Statistics,
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
    pub room_id: u128,
    pub msg: Vec<String>,
    pub color: i32,
    pub mode: String,
    pub room_type: i32,
    pub jumpfrom: i32,
    pub replay_dmid: Option<String>,
    pub statistics: Statistics,
    pub click_time: i32,
    pub visit_id: Option<String>,
}

impl Config {
    pub async fn new() -> Self {
        let yaml_str = include_str!("../config/config.yaml");
        let loader: Loader = serde_yaml::from_str(yaml_str).unwrap();
        Self {
            room_id: loader.room.room_id,
            msg: loader.room.msg,
            color: loader.room.color,
            room_type: loader.room.room_type,
            jumpfrom: loader.room.jumpfrom,
            replay_dmid: loader.room.replay_dmid,
            statistics: loader.room.statistics,
            click_time: loader.room.click_time,
            visit_id: loader.room.visit_id,
            mode: loader.room.mode,
        }
    }
}
