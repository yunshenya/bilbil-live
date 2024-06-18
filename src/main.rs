#![feature(duration_constructors)]

use std::time::Duration;
use log::info;
use tokio::{join, task};
use tokio::time::sleep;
use crate::bil_log::init_log;
use crate::comment::Comment;

mod config;
mod bil_log;
mod like;
mod utils;
mod comment;

#[tokio::main]
async fn main() {
    init_log();
    let task1 = task::spawn(async {
        loop {
            let comment = Comment::new(&Comment::default()).await;
            let form = comment.build_form(Option::from(String::from("修炼"))).await;
            comment.send(form).await;
            info!("主人修炼发送成功了( •̀ ω •́ )y");
            sleep(Duration::from_mins(5)).await;
            let form1 = comment.build_form(Option::from(String::from("突破"))).await;
            comment.send(form1).await;
            info!("主人突破发送成功了( •̀ ω •́ )y")
        }
    });

    let task2 = task::spawn(async {
        loop{
            let comment = Comment::new(&Comment::default()).await;
            let form2 = comment.build_form(None).await;
            comment.send(form2).await;
            sleep(Duration::from_secs(5)).await;
        }
    });
    let (handle1, handle2) = join!(task2, task1);
    (handle1.unwrap(), handle2.unwrap());

}
