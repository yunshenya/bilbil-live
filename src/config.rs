use log::info;
use std::fs::{read_to_string, OpenOptions};
use std::io::{stdin, stdout, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::api::CONFIG_PATH;

#[derive(Serialize, Deserialize)]
struct Loader {
    room: RoomInfo,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
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
        let path = Path::new(CONFIG_PATH);
        if path.exists() {
            let yaml_str = read_to_string(CONFIG_PATH).unwrap();
            let loader: Loader = serde_yaml::from_str(&*yaml_str).unwrap();
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
        } else {
            let loader = Loader::default();
            let serialized = serde_yaml::to_string(&loader).unwrap();
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(CONFIG_PATH)
                .unwrap();
            file.write_all(serialized.as_bytes()).unwrap();
            Config {
                room_id: loader.room.room_id,
                msg: loader.room.msg,
                color: loader.room.color,
                mode: loader.room.mode,
                room_type: loader.room.room_type,
                jumpfrom: loader.room.jumpfrom,
                replay_dmid: loader.room.replay_dmid,
                statistics: loader.room.statistics,
                click_time: loader.room.click_time,
                visit_id: loader.room.visit_id,
            }
        }
    }
}

impl Default for Loader {
    fn default() -> Self {
        let mut room_id_str = String::new();
        print!("请输入直播间房间号(未填写则为默认): ");
        stdout().flush().unwrap();
        stdin().read_line(&mut room_id_str).unwrap();
        let room_id = room_id_str
            .trim()
            .parse::<u128>()
            .unwrap_or_else(|_| 30513598);
        info!("已创建配置文件");
        Self {
            room: RoomInfo {
                room_id,
                msg: vec![
                    "歡迎來到直播間~".to_string(),
                    "喜歡主播的點點關注，看看右下角歌單~".to_string(),
                    "喜歡主播的點點關注，看看右下角歌單~".to_string(),
                ],
                color: 16777215,
                mode: "1".to_string(),
                room_type: 0,
                jumpfrom: 71004,
                replay_dmid: None,
                statistics: Statistics::default(),
                click_time: 2,
                visit_id: None,
            },
        }
    }
}

impl Default for Statistics{
    fn default() -> Self {
        Self{
            app_id: 100,
            platform: 5,
        }
    }
}