use std::io::{stdout, Write};
use crate::plugin::comment::Comment;
use crate::plugin::like::LikeSend;
use crate::plugin::video::FlashVideoWatch;
use crate::util::error::BilCoreResult;
use log::{info, warn};
use std::sync::Arc;
use std::time::Duration;
use ansi_term::Color::{Blue, Purple};
use ansi_term::{Colour, Style};
use ansi_term::Colour::Red;
use chrono::Local;
use tokio::time::{sleep, interval};
use tokio::{self, join, task};

pub struct Task;

impl Task {
    pub async fn run() {
        let mut count = 0;
        let retry_limit = 10;
        let mut interval = interval(Duration::from_millis(1000));
        let mut like_count = 0;
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
                let fmt = "%Y年%m月%d日 %H:%M:%S";
                let now = Local::now().format(fmt);
                let pink = Colour::RGB(255, 192, 203); //粉色
                let app_name = Style::new().fg(pink).paint("bilbil-live");
                like_count += 1;
                stdout().flush().unwrap();
                print!("\r[{}] [{}] ({}): 已点赞{}个", Purple.paint(now.to_string()), Blue.paint("点赞中..."), app_name,Red.paint(like_count.to_string()));
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

    pub async fn run_video(bvid: &str) -> FlashVideoWatch {
        FlashVideoWatch::new(bvid).await
    }
}