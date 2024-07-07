use crate::arrangement::api::{ADD, DOSINGN, VIDEO_INFO};
use crate::logged::load_cookies::CookiesConfig;
use crate::util::utils::Utils;
use log::{error, info, warn};
use reqwest::header::{HeaderMap, CONTENT_TYPE};
use serde::Deserialize;

#[derive(Deserialize)]
struct DoSignJson {
    message: String,
}

#[derive(Deserialize)]
struct LiveAdd {
    message: String,
}

pub async fn live_add() {
    let mut headers = HeaderMap::new();
    let csrf = &*CookiesConfig::csrf();
    headers.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    let params = vec![("csrf", csrf)];
    let utils = Utils::new(ADD).await;
    match utils.post_with_form(params, headers).await {
        Ok(resp) => {
            let add = serde_json::from_str::<LiveAdd>(&resp.text().await.unwrap()).unwrap();
            let message = if add.message.eq("0") {
                "用户经验领取成功"
            } else {
                "用户经验已经领取"
            };
            info!("{}", message)
        }
        Err(err) => {
            warn!("{}", err)
        }
    }
}

pub async fn do_sign() {
    let util = Utils::new(DOSINGN).await;
    match util.send_get().await {
        Ok(resp) => {
            let do_sign_json =
                serde_json::from_str::<DoSignJson>(&resp.text().await.unwrap()).unwrap();
            info!("{}", do_sign_json.message)
        }
        Err(err) => {
            error!("签到失败: {}", err);
        }
    }
}

pub async fn get_video_info(bvid: &str) {
    let util = Utils::new(VIDEO_INFO).await;
    let params = vec![("bvid", bvid)];
    let response = util.sne_get(params).await;
    println!("{}", response.text().await.unwrap());
}
