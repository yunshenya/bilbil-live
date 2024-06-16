use log::{debug, error, trace, warn};
use crate::bil_log::init_log;

mod config;
mod bil_log;


#[tokio::main]
async fn main() {
    init_log();
    warn!("警告");
    debug!("调试");
    trace!("跟踪");
    error!("大错误")
}
