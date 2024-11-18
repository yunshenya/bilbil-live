use crate::arrangement::bil_log::init_log;
use crate::logged::login::Login;
use crate::plugin::sign::{do_sign, live_add};
use crate::util::error::BilCoreResult;
use crate::util::task::Task;
use ansi_term::Color::{Blue, Green, Red};
use log::error;
use std::io::{stdin, stdout, Write};
use std::process::exit;

mod arrangement;
mod logged;
mod plugin;
mod util;

#[tokio::main]
async fn main() -> BilCoreResult<()> {
    init_log();
    Login::new().await;
    live_add().await;
    do_sign().await;
    println!("{}", Green.paint("请选择想要实现的功能"));
    let options = ["直播间刷赞", "直播间刷弹幕", "视频刷赞"];
    for (index, options) in options.iter().enumerate() {
        println!(
            "{}: {} {}",
            Blue.paint("序号"),
            Red.paint((index + 1).to_string()),
            options
        )
    }
    let mut input = String::new();
    print!(
        "{} ({}{}{}): ",
        Green.paint("输入您的选择"),
        Red.paint("1"),
        Blue.paint("-"),
        Red.paint(options.len().to_string())
    );
    stdout().flush()?; // 确保提示信息被立即打印
                       // 读取用户输入
    stdin().read_line(&mut input).expect("Failed to read line");

    // 处理用户输入
    match input.trim().parse::<usize>() {
        Ok(choice) if choice >= 1 && choice <= options.len() => {
            println!(
                "{} {}",
                Green.paint("您选择了:"),
                Red.paint(options[choice - 1])
            );
            match choice {
                1 => {
                    Task::run().await;
                }
                2 => {
                    Task::run_live().await?;
                }
                3 => loop {
                    print!("{}", Green.paint("请输入视频bvid: "));
                    stdout().flush()?;
                    let mut bvid_str = String::new();
                    stdin().read_line(&mut bvid_str)?;
                    let bvid = bvid_str.trim();
                    if bvid.starts_with("BV") && bvid.len() == 12 {
                        Task::run_video(&bvid).await?;
                        break;
                    }
                },
                _ => {
                    error!("索引错误");
                    exit(0)
                }
            }
        }
        _ => {
            println!(
                "Invalid choice. Please enter a number between 1 and {}.",
                options.len()
            );
        }
    };
    Ok(())
}
