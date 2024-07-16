use bil_derive::Api;

pub const COOKIES_PATH: &str = "config/cookies.yaml";
pub const CONFIG_PATH: &str = "config/config.yaml";

#[derive(Api)]
#[api(endpoint("https://api.live.bilibili.com/xlive/web-ucenter/v1/sign/DoSign"))]
pub struct DoSign;

#[derive(Api)]
#[api(endpoint("https://api.bilibili.com/x/web-interface/nav"))]
pub struct Nav;

#[derive(Api)]
#[api(endpoint("https://api.bilibili.com/x/web-interface/view/conclusion/judge"))]
pub struct Judge;
#[derive(Api)]
#[api(endpoint("https://passport.bilibili.com/x/passport-login/web/qrcode/generate"))]
pub struct GetCodeUrl;

#[derive(Api)]
#[api(endpoint("https://passport.bilibili.com/x/passport-login/web/qrcode/poll"))]
pub struct ScanInfo;

#[derive(Api)]
#[api(endpoint("https://api.live.bilibili.com/msg/send"))]
pub struct CommentSendUrl;

#[derive(Api)]
#[api(endpoint(
    "https://api.live.bilibili.com/xlive/app-ucenter/v1/like_info_v3/like/likeReportV3"
))]
pub struct SendLikeUrl;

#[derive(Api)]
#[api(endpoint("https://api.bilibili.com/x/member/web/account"))]
pub struct GetAccount;

#[derive(Api)]
#[api(endpoint("https://api.live.bilibili.com/room/v1/Room/get_info"))]
pub struct GetLiveInfo;

#[derive(Api)]
#[api(endpoint("https://api.bilibili.com/x/vip/experience/add"))]
pub struct Add;

#[derive(Api)]
#[api(endpoint("https://api.bilibili.com/x/web-interface/view"))]
pub struct VideoInfo;
