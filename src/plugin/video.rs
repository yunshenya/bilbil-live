use log::info;
use serde::Deserialize;

use crate::arrangement::api::VideoInfo;
use crate::util::error::BilCoreResult;
use crate::util::utils::Utils;

#[derive(Deserialize, Debug)]
struct VideoJson {
    code: i32,
    message: String,
    data:  VideoData,
    ttl: u8,
}

#[derive(Deserialize, Debug)]
struct VideoData {
    title: String,
    aid: u64,
    owner: OwnerData,
    stat: StatData,
    cid: u64,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
struct StatData {
    view: u64,
    coin: u64,
    like: u64,
    dislike: u64,
    share: u64
}

#[derive(Deserialize)]
struct RespKey<'a> {
    w_rid: &'a str,
    wts: u64,
}

pub struct FlashVideoWatch<'a>{
    params: Vec<(&'a str, &'a str)>,
}

impl <'a>FlashVideoWatch<'a> {
    pub async fn new(bvid: &'a str) -> Self {
        println!("你输入的bvid为: {}", bvid);
        let params = vec![("bvid", bvid)];
        Self{ params }
    }

    pub async fn get_video_info(&self) -> BilCoreResult<()> {
        let util  = Utils::new(VideoInfo::get_api()).await;
        let result = util.sne_get_with_params(&self.params).await?;
        let video_data = result.json::<VideoJson>().await?;
        if video_data.code.eq(&0) {
            info!("视频标题: {:?}", video_data.data.title);
        }
        Ok(())
    }
}
