#![feature(duration_constructors)]

use crate::plugin::sign::{do_sign, get_video_info, live_add};
use arrangement::bil_log::init_log;
use log::{info, warn};
use logged::login::Login;
use plugin::comment::Comment;
use plugin::like::LikeSend;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tokio::{join, task};

mod arrangement;
mod logged;
mod plugin;
mod util;

#[tokio::main]
async fn main() {
    init_log();
    Login.new().await;
    live_add().await;
    do_sign().await;
    get_video_info("BV1wW421d7TZ").await;
    run_live().await;
}

async fn run_live() {
    let share_comment = Arc::new(Comment::new(&Comment::default()).await);
    let comment = Arc::clone(&share_comment);
    let task1 = task::spawn(async move {
        loop {
            let form = comment.build_form(Option::from(String::from("修炼"))).await;
            comment.send(form).await;
            info!("主人修炼发送成功了( •̀ ω •́ )y");
            sleep(Duration::from_mins(10)).await;
            let form1 = comment.build_form(Option::from(String::from("突破"))).await;
            comment.send(form1).await;
            info!("主人突破发送成功了( •̀ ω •́ )y")
        }
    });
    let comment2 = Arc::clone(&share_comment);
    let task2 = task::spawn(async move {
        loop {
            sleep(Duration::from_millis(5000)).await;
            let form2 = comment2.build_form(None).await;
            comment2.send(form2).await;
        }
    });

    let task3 = task::spawn(async {
        let mut is_like = true;
        loop {
            sleep(Duration::from_millis(1000)).await;
            if is_like {
                is_like = match LikeSend::new().await {
                    Ok(_) => {
                        info!("点赞成功");
                        true
                    }
                    Err(err) => {
                        warn!("点赞失败,错误码: {}", err);
                        false
                    }
                };
            }
        }
    });
    let _ = join!(task1, task2, task3);
}
