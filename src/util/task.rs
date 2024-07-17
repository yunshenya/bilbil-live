use crate::plugin::comment::Comment;
use crate::plugin::like::LikeSend;
use crate::plugin::video::FlashVideoWatch;
use crate::util::error::BilCoreResult;
use log::{info, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tokio::{join, task};

pub struct Task;

impl Task {
    pub async fn run(){
        let task = task::spawn(async {
            info!("点赞开始...");
            let mut count  = 0;
            loop {
                sleep(Duration::from_millis(1000)).await;
                match LikeSend::new().await {
                    Ok(_) => {
                        continue
                    }
                    Err(err) => {
                        count += 1;
                        if count < 10 {
                            warn!("{}" ,err);
                            info!("点赞继续, 第{count}次错误")
                        }else {
                            warn!("以重试{count}次, 点赞暂停");
                            break
                        }
                    }
                };
            }
        });
        task.await.unwrap();
    }

    pub async fn run_live() -> BilCoreResult<()> {
        let share_comment = Arc::new(Comment::new().await);
        let comment = Arc::clone(&share_comment);
        let task1 = task::spawn(async move {
            loop {
                let form = comment
                    .build_form(Option::from(String::from("修炼")))
                    .await
                    .unwrap_or_default();
                comment.send(form).await.unwrap_or_default();
                info!("主人修炼发送成功了( •̀ ω •́ )y");
                sleep(Duration::from_mins(10)).await;
                let form1 = comment
                    .build_form(Option::from(String::from("突破")))
                    .await
                    .unwrap_or_default();
                match comment.send(form1).await {
                    Ok(_) => {}
                    Err(err) => {
                        warn!("{}", err)
                    }
                };
                info!("主人突破发送成功了( •̀ ω •́ )y")
            }
        });
        let comment2 = Arc::clone(&share_comment);
        let task2 = task::spawn(async move {
            loop {
                sleep(Duration::from_millis(5000)).await;
                let form2 = comment2.build_form(None).await.unwrap_or_default();
                comment2.send(form2).await.err();
            }
        });

        let _ = join!(task1, task2);
        Ok(())
    }

    pub async fn run_video(bvid: &str) {
        FlashVideoWatch::new(bvid).await;
    }
}
