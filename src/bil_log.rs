use std::io::{stdout, Write};
use ansi_term::Color::{Blue, Green, Purple, Red, Yellow};
use chrono::Local;
use log::{info, Level, LevelFilter, Log, Metadata, Record, set_logger, set_max_level};
use log::Level::{Debug, Error, Info};

struct BilLog;


impl Log for BilLog{

    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let fmt = "%Y年%m月%d日 %H:%M:%S";
            let now = Local::now().format(fmt);
            match record.level() {
                Error => {
                    println!("{} {} {}", Red.paint(now.to_string()), Red.paint(record.level().as_str()), Red.paint(record.args().to_string()));
                }
                Level::Warn => {
                    println!("{} {} {}", Yellow.paint(now.to_string()), Yellow.paint(record.level().as_str()), Yellow.paint(record.args().to_string()));
                }
                Info => {
                    println!("{} {} {}", Purple.paint(now.to_string()), Blue.paint(record.level().as_str()), Green.paint(record.args().to_string()));
                }
                Debug => {
                    println!("{} {} {}", Blue.paint(now.to_string()), Yellow.paint(record.level().as_str()), Green.paint(record.args().to_string()));
                }
                Level::Trace => {
                    println!("{} {} {}", now, record.level(),record.args());
                }
            };
        }
    }

    fn flush(&self) {
        stdout().flush().unwrap();
    }
}
pub fn init_log(){
    match set_logger(&BilLog).map(|()| {
        set_max_level(LevelFilter::Info)
    }) {
        Ok(_) => {
            let banner = r#"
        _         _    _         _          _
( )     _ (_ ) ( )     _ (_ )       (_ )  _
| |_   (_) | | | |_   (_) | | ______ | | (_) _   _    __
| '_`\ | | | | | '_`\ | | | |(______)| | | |( ) ( ) /'__`\
| |_) )| | | | | |_) )| | | |        | | | || \_/ |(  ___/
(_,__/'(_)(___)(_,__/'(_)(___)      (___)(_)`\___/'`\____)
        作者: 云深不知处
    "#;
            let picture = Purple.paint(banner);
            println!("{}", picture);
            info!("日志初始化完成")
        }
        Err(err) => {
            println!("日志初始化失败 {}", err)}
    };
}