use crate::arrangement::api::VIDEO_INFO;
use crate::util::utils::Utils;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct VideoJson {
    data: VideoData,
}

#[derive(Deserialize, Debug)]
struct VideoData {
    title: String,
    aid: u64,
    owner: OwnerData,
    stat: StatData,
}

#[derive(Deserialize, Debug)]
struct OwnerData {
    mid: u64,
    name: String,
}

#[derive(Deserialize, Debug)]
struct StatData {
    aid: u64,
    view: u64,
    coin: u64,
    like: u64,
    dislike: u64,
}

pub struct FlashVideoWatch;

impl FlashVideoWatch {
    pub async fn new(bvid: &str) {
        FlashVideoWatch::get_video_info(bvid).await;
    }

    async fn get_video_info(bvid: &str) {
        let util = Utils::new(VIDEO_INFO).await;
        let params = vec![("bvid", bvid)];
        let response = util.sne_get(params).await;
        let video_json =
            serde_json::from_str::<VideoJson>(&response.text().await.unwrap()).unwrap();
        println!("{:?}", video_json.data.stat);
    }
}
