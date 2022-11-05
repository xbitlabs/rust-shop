use chrono::Local;
use hyper::{Client, Method, Request, Uri};
use hyper_tls::HttpsConnector;
use lazy_static::lazy_static;
use log::error;
use hyper::body::Buf;
use std::sync::Mutex;
use anyhow::anyhow;
use crate::app_config::load_mod_config;


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Wechat{
    pub app_id:String,
    pub app_secret:String,
}

#[derive(serde::Serialize,serde::Deserialize,PartialEq)]
pub struct WeChatAccessTokenResponse{
    pub access_token:Option<String>,
    pub expires_in:Option<i64>,
    pub errcode:Option<i64>,
    pub errmsg:Option<String>
}
#[derive(serde::Serialize,serde::Deserialize)]
pub struct WeChatMiniAppLoginResponse{
    pub session_key:Option<String>,
    pub unionid:Option<String>,
    pub errmsg:Option<String>,
    pub openid:Option<String>,
    pub errcode:Option<i64>,
}
#[derive(serde::Serialize,serde::Deserialize)]
pub struct Watermark {
    pub timestamp:i64,
    pub appid:String
}
#[derive(serde::Serialize,serde::Deserialize)]
pub struct PhoneInfo {
    pub phoneNumber:Option<String>,
    pub purePhoneNumber:Option<String>,
    pub countryCode:Option<i64>,
    pub watermark:Watermark
}
#[derive(serde::Serialize,serde::Deserialize)]
pub struct GetUserPhoneNumberResponse{
    pub errcode:Option<i64>,
    pub errmsg:Option<String>,
    pub phone_info:Option<PhoneInfo>,
}
#[derive(serde::Serialize,serde::Deserialize)]
pub struct WeChatUserInfo{
    pub openid:String,
    pub nickname:String,
    pub sex:i32,
    pub province:String,
    pub city:String,
    pub country:String,
    pub headimgurl:String,
    pub privilege:String,
    pub unionid:Option<String>,
    pub errcode:Option<String>,
    pub errmsg:Option<String>,
}


lazy_static! {
    static ref WE_CHAT_ACCESS_TOKEN_MANAGER: Mutex<WeChatAccessTokenManager> = Mutex::new(WeChatAccessTokenManager::new());
}
lazy_static! {
    ///
    /// 全局配置
    ///
    pub static ref WE_CHAT_CONFIG: Wechat = load_mod_config(String::from("wechat")).unwrap();
}
///token还剩下多少秒有效的时间就要去刷新token
const TOKEN_REMAINING_LIFETIME_SECONDS_REFRESH: i64 = 10 * 60;

///token有效期为7200s，
struct WeChatAccessTokenManager{
    access_token_created:Option<i64>,
    token:Option<WeChatAccessTokenResponse>,
}

impl WeChatAccessTokenManager {
    pub fn new()->Self{
        WeChatAccessTokenManager{
            access_token_created:None,
            token:None,
        }
    }
    ///如果已经快过期，就主动去刷新，提前10分钟去刷新
    pub async fn get_access_token(&mut self)->anyhow::Result<String>{
        //第一次请求，需要请求腾讯服务器获取新的
        if self.token == None || self.access_token_created == None {
            let token_response = self.request_access_token().await?;
            return self.get_token_from_response(token_response)
        }
        let elapsed_time =  Local::now().timestamp() - self.access_token_created.unwrap();
        //已过期
        if elapsed_time >= self.token.as_ref().unwrap().expires_in.unwrap() {
            let token_response = self.request_access_token().await?;
            return self.get_token_from_response(token_response)
        }else {
            //没有过期，还还没有到刷新时间，直接使用上一次的
            if self.token.as_ref().unwrap().expires_in.unwrap() - elapsed_time < TOKEN_REMAINING_LIFETIME_SECONDS_REFRESH{
                Ok(self.token.as_ref().unwrap().access_token.as_ref().unwrap().as_str().to_string())
            }else {
                //快到期限了，刷新token
                let token_response = self.request_access_token().await?;
                return self.get_token_from_response(token_response)
            }
        }
    }
    fn get_token_from_response(&mut self,token_response : WeChatAccessTokenResponse)->anyhow::Result<String>{
        return  match token_response.errcode {
            Some(_)=>{
                Err(anyhow!(token_response.errmsg.unwrap()))
            }
            None=>{
                self.token = Some(token_response);
                self.access_token_created = Some(Local::now().timestamp());
                Ok(self.token.as_ref().unwrap().access_token.as_ref().unwrap().as_str().to_string())
            }
        }
    }
    async fn request_access_token(&self)->anyhow::Result<WeChatAccessTokenResponse>{
        let wechat_config = &WE_CHAT_CONFIG;
        //let client = Client::new();
        let https = HttpsConnector::new();
        let client = Client::builder()
            .build::<_, hyper::Body>(https);
        let url = format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
                          wechat_config.app_id,
                          wechat_config.app_secret).parse().unwrap();
        println!("url = {}",url);
        let response_result = client.get(url).await?;
        let body = hyper::body::aggregate(response_result).await?;
        let access_token_response = serde_json::from_reader(body.reader())?;
        Ok(access_token_response)
    }
}
pub struct WeChatMiniAppService{

}
impl WeChatMiniAppService{
    pub fn new()->Self{
        WeChatMiniAppService{
        }
    }
    pub async fn get_access_token(&self)->anyhow::Result<String>{
        let lock_result = WE_CHAT_ACCESS_TOKEN_MANAGER.lock();
        match lock_result {
            Ok(mut result)=>{
                Ok(result.get_access_token().await?)
            }
            Err(error)=>{
                error!("获取微信access_token异常：{}",error);
                Err(anyhow!(error.to_string()))
            }
        }
    }
    //https://developers.weixin.qq.com/doc/offiaccount/OA_Web_Apps/Wechat_webpage_authorization.html#3
    //https://api.weixin.qq.com/sns/userinfo?access_token=ACCESS_TOKEN&openid=OPENID&lang=zh_CN
    pub async fn get_userinfo(&self,openid:String)->anyhow::Result<WeChatUserInfo>{
        let lock_result = WE_CHAT_ACCESS_TOKEN_MANAGER.lock();
        match lock_result {
            Ok(mut result)=>{
                let access_token = result.get_access_token().await?;
                let https = HttpsConnector::new();
                let client = Client::builder()
                    .build::<_, hyper::Body>(https);
                let url = format!("https://api.weixin.qq.com/sns/userinfo?access_token={}&openid={}&lang=zh_CN",
                                  access_token,
                                  openid).parse().unwrap();
                let response_result = client.get(url).await?;
                let body = hyper::body::aggregate(response_result).await?;
                let user_info = serde_json::from_reader(body.reader())?;
                Ok(user_info)
            }
            Err(error)=>{
                error!("获取微信用户信息异常：{}",error);
                Err(anyhow!(error.to_string()))
            }
        }

    }
    pub async fn login(&self,js_code:String)->anyhow::Result<WeChatMiniAppLoginResponse>{
        let wechat_config = &WE_CHAT_CONFIG;
        let https = HttpsConnector::new();
        let client = Client::builder()
            .build::<_, hyper::Body>(https);
        let url = format!("https://api.weixin.qq.com/sns/jscode2session?appid={}&secret={}&js_code={}&grant_type=authorization_code",
                          wechat_config.app_id,
                          wechat_config.app_secret,js_code).parse().unwrap();
        println!("wechat login url:{}",url);
        let response_result = client.get(url).await?;

        let body = hyper::body::aggregate(response_result).await?;
        let login_response = serde_json::from_reader(body.reader())?;
        Ok(login_response)
    }
    pub async fn get_user_phone_number(&self,code:String)->anyhow::Result<GetUserPhoneNumberResponse>{
        let https = HttpsConnector::new();
        let client = Client::builder()
            .build::<_, hyper::Body>(https);
        let lock_result = WE_CHAT_ACCESS_TOKEN_MANAGER.lock();
        match lock_result {
            Ok(mut result)=>{
                let access_token = result.get_access_token().await?;
                let url:Uri = format!("https://api.weixin.qq.com/wxa/business/getuserphonenumber?access_token={}", access_token).parse().unwrap();
                let req = Request::builder().method(Method::POST)
                    .uri(url)
                    .body(("{".to_string() + format!("\"code\":\"{}\"",code).to_string().as_str() + "}").into()).unwrap();
                let response_result = client.request(req).await?;
                let body = hyper::body::aggregate(response_result).await?;
                let phone_number_response = serde_json::from_reader(body.reader())?;
                Ok(phone_number_response)
            }
            Err(e)=>{
                error!("获取微信用户手机号异常：{}",e);
                Err(anyhow!(e.to_string()))
            }
        }

    }
}
macro_rules! aw {
  ($e:expr) => {
      tokio_test::block_on($e)
  };
}
#[test]
fn test_wechat_api(){
    let api = WeChatMiniAppService::new();
    let result = api.get_access_token();
    let result1 = aw!(result);
    println!("{:?}",result1);
}