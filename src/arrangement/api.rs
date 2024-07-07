//获取登录验证码
pub const GET_CODE_URL: &str = "https://passport.bilibili.com/x/passport-login/web/qrcode/generate";
//验证二维码是否有效
pub const SCAN_INFO: &str = "https://passport.bilibili.com/x/passport-login/web/qrcode/poll";
//配置目录
pub const COOKIES_PATH: &str = "config/cookies.yaml";
//发送直播间消息
pub const COMMENT_SEND_URL: &str = "https://api.live.bilibili.com/msg/send";
//发送直播点赞
pub const SEND_LIKE_URL: &str =
    "https://api.live.bilibili.com/xlive/app-ucenter/v1/like_info_v3/like/likeReportV3";
//配置文件路径
pub const CONFIG_PATH: &str = "config/config.yaml";
//获取个人信息
pub const GET_ACCOUNT: &str = "https://api.bilibili.com/x/member/web/account";
//获取直播间信息
pub const GET_LIVE_INFO: &str = "https://api.live.bilibili.com/room/v1/Room/get_info";
//每日签到
pub const ADD: &str = "https://api.bilibili.com/x/vip/experience/add";
//视频信息 bvid
pub const VIDEO_INFO: &str = "https://api.bilibili.com/x/web-interface/view";
//直播签到
pub const DOSINGN: &str = "https://api.live.bilibili.com/xlive/web-ucenter/v1/sign/DoSign";
