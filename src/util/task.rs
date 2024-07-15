use crate::plugin::comment::Comment;
use crate::plugin::like::LikeSend;
use crate::plugin::sign::{do_sign, live_add};
use crate::plugin::video::FlashVideoWatch;
use log::{error, info, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tokio::{join, task};

pub struct Task;

impl Task {
    pub async fn run() {
        live_add().await;
        do_sign().await;
        task::spawn(async {
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
                            error!("点赞失败, {}", err);
                            warn!("点赞已停止");
                            false
                        }
                    };
                }else {
                    break
                }
            }
        })
        .await
        .expect("点赞任务执行失败");
    }

    pub async fn run_live() {
        let share_comment = Arc::new(Comment::new().await);
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

        let _ = join!(task1, task2);
    }

    pub async fn run_video(bvid: &str) {
        FlashVideoWatch::new(bvid).await;
    }
}