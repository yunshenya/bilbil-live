#![feature(duration_constructors)]

use crate::bil_log::init_log;
use crate::comment::Comment;
use crate::like::LikeSend;
use crate::login::Login;
use log::info;
use std::time::Duration;
use tokio::time::sleep;
use tokio::{join, task};

mod api;
mod bil_log;
mod comment;
mod config;
mod like;
mod login;
mod utils;

mod load_cookies;

#[tokio::main]
async fn main() {
    init_log();
    Login.new().await;
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
        loop {
            sleep(Duration::from_millis(5000)).await;
            let comment = Comment::new(&Comment::default()).await;
            let form2 = comment.build_form(None).await;
            comment.send(form2).await;
        }
    });

    let task3 = task::spawn(async {
        loop {
            sleep(Duration::from_millis(1000)).await;
            LikeSend::new().await;
        }
    });
    let (handle1, handle2, handle3) = join!(task1, task2, task3);
    (handle1.unwrap(), handle2.unwrap(), handle3.unwrap());
}
