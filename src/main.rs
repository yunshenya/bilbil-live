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
    Comment.new().await;
}
