use crate::api::{FILE_PATH, GET_CODE_URL, SCAN_INFO};
use crate::load_cookies::CookiesConfig;
use log::{info, warn};
use qrcode::render::unicode;
use qrcode::QrCode;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

pub struct Login;

pub enum Statue {
    Success,
    Expired,
    NotConfirmed,
    NotScanned,
    Unknown,
}

#[derive(Serialize, Deserialize)]
struct CodeResultData {
    code: u32,
    message: String,
    //并不是真正意义上的url
    url: String,
    refresh_token: String,
}

#[derive(Deserialize, Serialize)]
struct CodeResult {
    data: CodeResultData,
}

#[derive(Serialize, Deserialize)]
struct QrData {
    url: String,
    qrcode_key: String,
}

#[derive(Serialize, Deserialize)]
struct Qrcode {
    data: QrData,
}
impl From<i32> for Statue {
    fn from(code: i32) -> Self {
        match code {
            0 => Statue::Success,
            86038 => Statue::Expired,
            86090 => Statue::NotConfirmed,
            86101 => Statue::NotScanned,
            _ => Statue::Unknown,
        }
    }
}

impl Login {
    pub async fn new(&self) {
        let path = Path::new(FILE_PATH);
        if path.exists() {
            let config = CookiesConfig::default();
            if config.is_login {
                println!("{}", config.cookies);
            } else {
                self.qrcode().await;
            }
        } else {
            let dir = FILE_PATH.split("/").next().unwrap();
            create_dir_all(dir).unwrap();
            info!("创建了配置文件: {}", dir);
            self.qrcode().await;
        }
    }

    async fn qrcode(&self) {
        let client = Client::new();
        let qr_response = client.get(GET_CODE_URL).send().await.unwrap();
        let qrcode: Qrcode = serde_json::from_str(&*qr_response.text().await.unwrap()).unwrap();
        let code = QrCode::new(&qrcode.data.url).unwrap();
        info!("正在使用二维码登录，已生成二维码");
        println!("{}", code.render::<unicode::Dense1x2>().build());
        let mut is_first = true;
        let mut is_confirmed_first = true;
        loop {
            let params = [("qrcode_key", &qrcode.data.qrcode_key)];
            let is_scan = client.get(SCAN_INFO).query(&params).send().await.unwrap();
            let (headers, text) = (is_scan.headers().clone(), is_scan.text().await.unwrap());
            let scan_info: CodeResult = serde_json::from_str(&*text).unwrap();
            match (scan_info.data.code as i32).into() {
                Statue::Success => {
                    info!("{}", "登录成功");
                    for (keys, value) in headers {
                        match keys {
                            None => {
                                println!("{:?}", value);
                            }
                            Some(key) => {
                                println!("{}", key);
                            }
                        }
                    }
                    let config = CookiesConfig {
                        refresh_token: scan_info.data.refresh_token,
                        cookies: scan_info.data.url,
                        is_login: true,
                    };
                    let config_str = serde_yaml::to_string(&config).unwrap();
                    let mut file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .truncate(true)
                        .open(FILE_PATH)
                        .unwrap();
                    file.write_all(config_str.as_bytes()).unwrap();
                    info!("登录信息保存成功");
                    break;
                }
                Statue::Expired => {
                    warn!("{}", scan_info.data.message);
                    info!("请重新扫描二维码");
                    let future = Box::pin(self.qrcode()); // 对递归调用进行盒装箱
                    future.await; // 等待未来完成
                }
                Statue::NotConfirmed => {
                    if is_confirmed_first {
                        info!("请在手机端确认登录");
                        is_confirmed_first = false
                    }
                }
                Statue::NotScanned => {
                    if is_first {
                        info!("请打开bilbil移动端扫描二维码登录");
                        is_first = false
                    }
                }
                Statue::Unknown => warn!("未知错误"),
            }
            //多久轮询一次
            sleep(Duration::from_millis(5000 / 3)).await;
        }
    }
}
