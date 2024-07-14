use crate::arrangement::api::{JUDGE, NAV, VIDEO_INFO};
use crate::util::utils::Utils;
use serde::Deserialize;
use std::process::Command;

#[derive(Deserialize)]
struct VideoJson {
    data: VideoData,
}

#[derive(Deserialize)]
struct VideoData {
    title: String,
    aid: u64,
    owner: OwnerData,
    stat: StatData,
    cid: u64,
}

#[derive(Deserialize)]
struct OwnerData {
    mid: u64,
    name: String,
}

#[derive(Deserialize)]
struct WbiData {
    wbi_img: WbiImg,
}

#[derive(Deserialize)]
struct WbiImg {
    img_url: String,
    sub_url: String,
}
#[derive(Deserialize)]
struct WbiImgJson {
    data: WbiData,
}

#[derive(Deserialize)]
struct StatData {
    view: u64,
    coin: u64,
    like: u64,
    dislike: u64,
}

#[derive(Deserialize)]
struct RespKey<'a> {
    w_rid: &'a str,
    wts: u64,
}

pub struct FlashVideoWatch;

impl FlashVideoWatch {
    pub async fn new(bvid: &str) -> Self {
        FlashVideoWatch::get_video_info(bvid).await;
        Self
    }

    async fn get_video_info(bvid: &str) {
        let util = Utils::new(VIDEO_INFO).await;
        let params = vec![("bvid", bvid)];
        let response = util.sne_get(params).await;
        let video_json =
            serde_json::from_str::<VideoJson>(&response.text().await.unwrap()).unwrap();
        println!("投币量为: {:?}", video_json.data.stat.coin);
        println!("点踩: {}", video_json.data.stat.dislike);
        println!("点赞数为: {}", video_json.data.stat.like);
        println!("播放量: {}", video_json.data.stat.view);
        println!("cid: {}", video_json.data.cid);
        let cid = &*video_json.data.cid.to_string();
        println!("标题: {}", video_json.data.title);
        println!("aid: {}", video_json.data.aid);
        println!("用户名: {}", video_json.data.owner.name);
        println!("mid: {}", video_json.data.owner.mid);
        let up_mid = &*video_json.data.owner.mid.to_string();
        FlashVideoWatch::nav(bvid, cid, up_mid).await;
    }

    async fn nav(bvid: &str, cid: &str, up_mid: &str) {
        let util = Utils::new(NAV).await;
        let result = util.send_get().await.unwrap();
        let json = serde_json::from_str::<WbiImgJson>(&result.text().await.unwrap()).unwrap();
        let imgkey = json
            .data
            .wbi_img
            .img_url
            .trim_end_matches(".png")
            .split("/")
            .last()
            .unwrap();
        let subkey = json
            .data
            .wbi_img
            .sub_url
            .trim_end_matches(".png")
            .split("/")
            .last()
            .unwrap();
        let output = Command::new("node")
            .args(["./app.js", imgkey, subkey])
            .output();
        match output {
            Ok(rep) => {
                let str = rep.stdout;
                let string = String::from_utf8(str).unwrap();
                let resp_key = serde_json::from_str::<RespKey>(&string).unwrap();
                let wts = &*resp_key.wts.to_string();
                let w_rid = &*resp_key.w_rid.to_string();
                let utils_nav = Utils::new(JUDGE).await;
                let params = vec![
                    ("bvid", bvid),
                    ("cid", cid),
                    ("up_mid", up_mid),
                    ("web_location", "333.788"),
                    ("w_rid", w_rid),
                    ("wts", wts),
                ];
                let response = utils_nav.sne_get(params).await;
                println!("{}", response.text().await.unwrap());
            }
            Err(_) => {
                println!("出错了")
            }
        }
    }
}
