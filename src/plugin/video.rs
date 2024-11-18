use log::info;
use serde::Deserialize;
use std::cmp::PartialEq;

use crate::arrangement::api::{OnlineTotal, VideoInfo};
use crate::util::error::BilCoreResult;
use crate::util::utils::Utils;

#[derive(Deserialize, Debug)]
struct VideoJson {
    code: i32,
    data: VideoData,
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
    share: u64,
}

#[derive(Deserialize)]
struct RespKey<'a> {
    w_rid: &'a str,
    wts: u64,
}

#[derive(Deserialize, Debug)]
struct VideoTotal {
    code: u8,
    data: VideoTotalData,
}

#[derive(Deserialize, Debug)]
struct VideoTotalData {
    total: String,
    count: String,
}

pub struct FlashVideoWatch<'a> {
    params: Vec<(&'a str, &'a str)>,
}

impl PartialEq for VideoTotal {
    fn eq(&self, other: &Self) -> bool {
        self.data.count == other.data.count
    }
}

impl<'a> FlashVideoWatch<'a> {
    pub async fn new(bvid: &'a str) -> Self {
        println!("你输入的bvid为: {}", bvid);
        let params = vec![("bvid", bvid)];
        Self { params }
    }

    pub async fn get_video_info(&self) -> BilCoreResult<()> {
        let util = Utils::new(VideoInfo::get_api()).await;
        let result = util.sne_get_with_params(&self.params).await?;
        let video_data = result.json::<VideoJson>().await?;
        if video_data.code.eq(&0) {
            info!("视频标题: {:?}", video_data.data.title);
            info!(
                "视频详细信息: 点赞数 {} 分享数: {} 硬币数: {} 预览 {}",
                video_data.data.stat.like,
                video_data.data.stat.share,
                video_data.data.stat.coin,
                video_data.data.stat.view
            );
            self.get_video_total(video_data.data.cid).await?;
        };
        Ok(())
    }

    async fn get_video_total(&self, cid: u64) -> BilCoreResult<()> {
        let cid_str = &cid.to_string();
        let cid_str_t = cid_str.trim();
        let util = Utils::new(OnlineTotal::get_api()).await;
        let mut params_c = self.params.clone();
        params_c.push(("cid", cid_str_t));
        let mut last_data: Option<VideoTotal> = None;
        loop {
            let result = util.sne_get_with_params(&params_c).await?;
            let total = result.json::<VideoTotal>().await?;
            if total.code.eq(&0) {
                if let Some(private_data) = last_data {
                    if total != private_data {
                        info!(
                            "当前视频观看人数: {}  web端观看人数: {}",
                            total.data.total, total.data.count
                        );
                    };
                };
                last_data = Some(total);
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }
}
