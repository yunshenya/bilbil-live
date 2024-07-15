use crate::arrangement::api::{Add, DoSign};
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
    let utils = Utils::new(Add::get_api()).await;
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
    let util = Utils::new(DoSign::get_api()).await;
    match util.send_get().await {
        Ok(resp) => {
            let do_sign_json =
                serde_json::from_str::<DoSignJson>(&resp.text().await.unwrap()).unwrap();
            let do_message = if do_sign_json.message.eq("0") {
                "直播奖励领取成功"
            } else {
                &do_sign_json.message.to_string()
            };
            info!("{}", do_message)
        }
        Err(err) => {
            error!("签到失败: {}", err);
        }
    }
}
