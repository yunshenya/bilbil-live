use serde::Deserialize;

#[derive(Deserialize)]
struct Loader{
    room:RoomInfo,
    userinfo:Cookies
}

#[derive(Deserialize)]
struct RoomInfo{
    #[serde(rename="roomID")]
    room_id:i128
}

#[derive(Deserialize)]
struct Cookies{
    cookies:String,
    uid:i128,
    csrf:String
}

#[warn(unused_imports)]
pub struct Config{
    pub cookies:String,
    pub room_id:i128,
    pub uid:i128,
    pub csrf:String
}

impl Config{
    pub fn new() -> Self{
        let yaml_str = include_str!("../config/config.yaml");
        let loader:Loader = serde_yaml::from_str(yaml_str).unwrap();
        Self{
            cookies:loader.userinfo.cookies,
            room_id: loader.room.room_id,
            uid: loader.userinfo.uid,
            csrf: loader.userinfo.csrf,
        }
    }
}