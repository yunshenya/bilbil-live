use ansi_term::Color::{Blue, Green, Purple, Red, Yellow};
use chrono::Local;
use log::Level::{Debug, Error, Info};
use log::{info, set_logger, set_max_level, Level, LevelFilter, Log, Metadata, Record};
use std::io::{stdout, Write};

struct BilLog;

impl Log for BilLog {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let fmt = "%Y年%m月%d日 %H:%M:%S";
            let now = Local::now().format(fmt);
            match record.level() {
                Error => {
                    println!(
                        "{} [{}] | bilbil-live | {}",
                        Red.paint(now.to_string()),
                        Red.paint("杂鱼"),
                        Red.paint(record.args().to_string())
                    );
                }
                Level::Warn => {
                    println!(
                        "{} [{}] | bilbil-live| {}",
                        Yellow.paint(now.to_string()),
                        Yellow.paint("笨蛋"),
                        Yellow.paint(record.args().to_string())
                    );
                }
                Info => {
                    println!(
                        "{} [{}] | bilbil-live | {}",
                        Purple.paint(now.to_string()),
                        Blue.paint("摸鱼"),
                        Green.paint(record.args().to_string())
                    );
                }
                Debug => {
                    println!(
                        "{} [{}] | bilbil-live | {}",
                        Blue.paint(now.to_string()),
                        Yellow.paint("雌小鬼"),
                        Green.paint(record.args().to_string())
                    );
                }
                Level::Trace => {
                    println!("{} [{}] | bilbil-live | {}", now, record.level(), record.args());
                }
            };
        }
    }

    fn flush(&self) {
        stdout().flush().unwrap();
    }
}
pub fn init_log() {
    match set_logger(&BilLog).map(|()| set_max_level(LevelFilter::Info)) {
        Ok(_) => {
            let banner = format!(
                r#"
        _         _    _         _          _
( )     _ (_ ) ( )     _ (_ )       (_ )  _
| |_   (_) | | | |_   (_) | | ______ | | (_) _   _    __
| '_`\ | | | | | '_`\ | | | |(______)| | | |( ) ( ) /'__`\
| |_) )| | | | | |_) )| | | |        | | | || \_/ |(  ___/
(_,__/'(_)(___)(_,__/'(_)(___)      (___)(_)`\___/'`\____)
                    {}
    "#,
                Yellow.paint("作者: 云深不知处")
            );
            let picture = Purple.paint(banner);
            println!("{}", picture);
            info!("日志初始化完成")
        }
        Err(err) => {
            println!("日志初始化失败 {}", err)
        }
    };
}
