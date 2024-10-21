use crate::arrangement::api::{GetAccount, GetCodeUrl, PathInfo, ScanInfo};
use crate::logged::load_cookies::CookiesConfig;
use log::{error, info, warn};
use qrcode::render::unicode;
use qrcode::QrCode;
use reqwest::header::COOKIE;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::time::Duration;
use tokio::time::sleep;

pub struct Login;

pub enum Statue {
    Success,
    Expired,
    NotConfirmed,
    NotScanned,
    Unknown
}

#[derive(Serialize, Deserialize)]
struct CodeResultData {
    code: i32,
    message: String,
    //并不是真正意义上的url
    url: String,
    refresh_token: String
}

#[derive(Deserialize, Serialize)]
struct CodeResult {
    data: CodeResultData
}

#[derive(Serialize, Deserialize)]
struct QrData {
    url: String,
    qrcode_key: String
}

#[derive(Deserialize, Serialize)]
struct AccountData {
    mid: u128,
    uname: String,
    sign: String
}

#[derive(Serialize, Deserialize)]
struct Account {
    data: AccountData
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
            _ => Statue::Unknown
        }
    }
}

impl Login {
    pub async fn new() -> Self {
        let path = Path::new(CookiesConfig::PATH);
        if path.exists() {
            let config = CookiesConfig::default();
            if config.is_login {
                info!("已找到配置文件，登录成功");
                Self
            } else {
                Login::qrcode().await;
                Self
            }
        } else {
            let dir = path.to_str().unwrap().split("/").next().unwrap();
            create_dir_all(dir).unwrap();
            info!("创建cookie文件: {}", dir);
            Login::qrcode().await;
            Self
        }
    }

    async fn qrcode() {
        let client = Client::new();
        match client.get(GetCodeUrl::get_api()).send().await {
            Ok(qr_response) => {
                let qrcode: Qrcode =
                    serde_json::from_str(&qr_response.text().await.unwrap()).unwrap();
                let code = QrCode::new(&qrcode.data.url).unwrap();
                info!("正在使用二维码登录，已生成二维码");
                println!("{}", code.render::<unicode::Dense1x2>().build());
                let mut is_first = true;
                let mut is_confirmed_first = true;
                loop {
                    let params = [("qrcode_key", &qrcode.data.qrcode_key)];
                    let is_scan = client
                        .get(ScanInfo::get_api())
                        .query(&params)
                        .send()
                        .await
                        .unwrap();
                    let (cookie, text) = (
                        is_scan
                            .cookies()
                            .map(|c| format!("{}={}", c.name(), c.value()))
                            .collect::<Vec<_>>()
                            .join("; "),
                        is_scan.text().await.unwrap(),
                    );
                    let scan_info = serde_json::from_str::<CodeResult>(&text).unwrap();
                    match scan_info.data.code.into() {
                        Statue::Success => {
                            info!("{}", "登录成功");
                            let resp = client
                                .get(scan_info.data.url)
                                .header(COOKIE, &cookie)
                                .send()
                                .await
                                .unwrap();
                            let cookies_str = resp
                                .cookies()
                                .map(|x| format!("{}={}", x.name(), x.value()))
                                .collect::<Vec<_>>()
                                .join("; ");
                            let cookies = format!("{}; {}", cookies_str, cookie);
                            let account_resp = client
                                .get(GetAccount::get_api())
                                .header(COOKIE, &cookies)
                                .send()
                                .await
                                .unwrap();
                            let account: Account =
                                serde_json::from_str(&account_resp.text().await.unwrap()).unwrap();
                            let config = CookiesConfig {
                                refresh_token: scan_info.data.refresh_token,
                                cookies,
                                is_login: true,
                                uid: account.data.mid,
                            };
                            let config_str = serde_yaml::to_string(&config).unwrap();
                            let mut file = OpenOptions::new()
                                .create(true)
                                .write(true)
                                .truncate(true)
                                .open(CookiesConfig::PATH)
                                .unwrap();
                            file.write_all(config_str.as_bytes()).unwrap();
                            info!("用户名: {} 签名: {}", account.data.uname, account.data.sign);
                            info!("登录信息保存成功");
                            break;
                        }
                        Statue::Expired => {
                            warn!("{}", scan_info.data.message);
                            info!("请重新扫描二维码");
                            Box::pin(Self::qrcode()).await; // 对递归调用进行盒装箱
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
            Err(_) => {
                error!("网络连接错误，请检查网络连接");
                exit(0);
            }
        };
    }
}
