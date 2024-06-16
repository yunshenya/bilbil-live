use std::io::{stdout, Write};
use ansi_term::Color::{Blue, Green, Purple, Red, Yellow};
use chrono::Local;
use log::{info, Level, LevelFilter, Log, Metadata, Record, set_logger, set_max_level};
use log::Level::{Debug, Error, Info};

struct BilLog;


impl Log for BilLog{

    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Debug
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
        set_max_level(LevelFilter::Debug)
    }) {
        Ok(_) => {info!("日志初始化完成")}
        Err(err) => {
            println!("日志初始化失败 {}", err)}
    };
}