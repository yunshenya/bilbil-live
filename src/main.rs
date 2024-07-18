use crate::arrangement::bil_log::init_log;
use crate::logged::login::Login;
use crate::plugin::sign::{do_sign, live_add};
use crate::util::task::Task;

mod arrangement;
mod logged;
mod plugin;
mod util;

#[tokio::main]
async fn main() {
    init_log();
    Login::new().await;
    live_add().await;
    do_sign().await;
    Task::run().await;
    Task::run_live().await.unwrap();
}
