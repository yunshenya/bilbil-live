use crate::plugin::comment::Comment;
use crate::plugin::like::LikeSend;
use crate::plugin::video::FlashVideoWatch;
use crate::util::error::BilCoreResult;
use log::{info, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{sleep, interval};
use tokio::{self, join, task};

pub struct Task;

impl Task {
    pub async fn run() {
        let mut count = 0;
        let retry_limit = 10;
        let mut interval = interval(Duration::from_millis(1000));

        info!("点赞开始...");
        loop {
            interval.tick().await;
            if let Err(err) = LikeSend::new().await {
                count += 1;
                warn!("点赞失败: {}", err);
                if count >= retry_limit {
                    warn!("已重试{}次，停止点赞任务", retry_limit);
                    break;
                } else {
                    info!("第{count}次错误，将继续尝试...");
                }
            } else {
                // 成功时无需操作，直接进入下一次循环
                continue
            }
        }
    }

    pub async fn run_live() -> BilCoreResult<()> {
        let share_comment = Arc::new(Comment::new().await);

        let task1 = {
            let comment = Arc::clone(&share_comment);
            task::spawn(async move {
                let messages = vec!["修炼", "突破"];
                let mut iter = messages.into_iter().cycle();

                loop {
                    let message = iter.next().unwrap();
                    let form = comment.build_form(Some(message.to_string())).await.unwrap_or_default();
                    comment.send(form).await.unwrap_or_default();
                    info!("消息 '{}' 发送成功了( •̀ ω •́ )y", message);
                    sleep(Duration::from_secs(600)).await; // 10分钟转换为秒
                }
            })
        };

        let task2 = {
            let comment = Arc::clone(&share_comment);
            task::spawn(async move {
                let mut interval = interval(Duration::from_millis(5000));
                loop {
                    interval.tick().await;
                    let form = comment.build_form(None).await.unwrap_or_default();
                    if let Err(err) = comment.send(form).await {
                        warn!("{}", err);
                    }
                }
            })
        };

        let _ = join!(task1, task2);
        Ok(())
    }

    pub async fn run_video(bvid: &str) -> BilCoreResult<()> {
        FlashVideoWatch::new(bvid).await;
        Ok(())
    }
}