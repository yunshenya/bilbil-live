#![feature(duration_constructors)]

use crate::arrangement::bil_log::init_log;
use crate::logged::login::Login;
use crate::util::task::Task;

mod arrangement;
mod logged;
mod plugin;
mod util;

#[tokio::main]
async fn main() {
    init_log();
    Login::new().await;
    Task::run().await
}
