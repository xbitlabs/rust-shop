use crate::RequestCtx;

use chrono::Local;
use log::error;
use redis::RedisResult;

use serde_json::Error;

use std::collections::HashMap;

use uuid::Uuid;

pub trait Session {
    fn get_last_activity(&mut self) -> i64;
    fn set_last_activity(&mut self, last_activity: i64);
    fn get_session_id(&self) -> &String;
    fn set_session_id(&mut self, session_id: String);
    fn is_new(&self) -> bool;
    fn insert_or_update<T>(&mut self, key: String, value: &T)
    where
        T: 'static + serde::Serialize + for<'a> serde::Deserialize<'a> + Send + Sync;
    fn get<T>(&self, key: String) -> Option<T>
    where
        T: 'static + serde::Serialize + for<'a> serde::Deserialize<'a> + Send + Sync;
    fn remove<T: 'static + Sync + Send>(&mut self, key: String) -> Option<T>
    where
        T: 'static + serde::Serialize + for<'a> serde::Deserialize<'a> + Send + Sync;
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct DefaultSession {
    last_activity: i64,
    new: bool,
    session_id: String,
    inner: HashMap<String, String>,
}

impl Default for DefaultSession {
    fn default() -> Self {
        DefaultSession {
            last_activity: Local::now().timestamp_millis(),
            new: false,
            session_id: "".to_string(),
            inner: Default::default(),
        }
    }
}
impl Session for DefaultSession {
    fn get_last_activity(&mut self) -> i64 {
        self.last_activity
    }
    fn set_last_activity(&mut self, last_activity: i64) {
        self.last_activity = last_activity;
    }
    fn get_session_id(&self) -> &String {
        &self.session_id
    }

    fn set_session_id(&mut self, session_id: String) {
        self.session_id = session_id;
    }

    fn is_new(&self) -> bool {
        self.new
    }
    fn insert_or_update<T>(&mut self, key: String, value: &T)
    where
        T: 'static + serde::Serialize + for<'a> serde::Deserialize<'a> + Send + Sync,
    {
        self.inner
            .insert(key, serde_json::to_string(&value).unwrap());
    }
    fn get<T>(&self, key: String) -> Option<T>
    where
        T: 'static + serde::Serialize + for<'a> serde::Deserialize<'a> + Send + Sync,
    {
        let result = self.inner.get(&*key);
        return match result {
            None => None,
            Some(json) => {
                let parse: Result<T, Error> = serde_json::from_str(json.as_str());
                return match parse {
                    Ok(obj) => Some(obj),
                    Err(_) => None,
                };
            }
        };
    }
    fn remove<T: 'static>(&mut self, key: String) -> Option<T>
    where
        T: 'static + serde::Serialize + for<'a> serde::Deserialize<'a> + Send + Sync,
    {
        let result = self.inner.remove(&*key);
        return match result {
            None => None,
            Some(json) => {
                let parse: Result<T, Error> = serde_json::from_str(json.as_str());
                return match parse {
                    Ok(obj) => Some(obj),
                    Err(_) => None,
                };
            }
        };
    }
}
#[async_trait::async_trait]
pub trait SessionStorage {
    async fn insert_or_update(&mut self, value: &DefaultSession) -> bool;
    async fn get(&self, session_id: String) -> Option<DefaultSession>;
    async fn remove(&mut self, session_id: String) -> bool;
}
pub struct RedisSession;
#[async_trait::async_trait]
impl SessionStorage for RedisSession {
    async fn insert_or_update(&mut self, value: &DefaultSession) -> bool {
        let key = String::from("session:") + value.session_id.as_str();
        let result = crate::redis::set(key.as_str(), value).await;
        return match result {
            Ok(_) => true,
            Err(err) => {
                error!("??????session?????????{}", err);
                false
            }
        };
    }

    async fn get(&self, key: String) -> Option<DefaultSession> {
        let key = String::from("session:") + key.as_str();
        let result: RedisResult<DefaultSession> = crate::redis::get(key.as_str()).await;
        return match result {
            Ok(session) => Some(session),
            Err(err) => {
                error!("??????session?????????{}", err);
                None
            }
        };
    }

    async fn remove(&mut self, key: String) -> bool {
        let key = String::from("session:") + key.as_str();
        crate::redis::remove(key.as_str()).await
    }
}
#[async_trait::async_trait]
pub trait SessionManager<T: Session> {
    async fn session_for_request(&mut self, req: &RequestCtx) -> T;
    async fn generate_session_id(&self, req: &RequestCtx) -> String;
    async fn save_session(&mut self, req: &mut RequestCtx);
}
pub struct DefaultSessionManager {
    session_storage: Box<dyn SessionStorage + Send + Sync>,
}

impl DefaultSessionManager {
    pub fn new(session_storage: Box<dyn SessionStorage + Send + Sync>) -> Self {
        DefaultSessionManager { session_storage }
    }
    async fn generate_new_session(&mut self, req: &RequestCtx) -> DefaultSession {
        let new_session_id = self.generate_session_id(req).await;
        let session = DefaultSession {
            last_activity: Local::now().timestamp_millis(),
            new: true,
            session_id: new_session_id.clone(),
            inner: HashMap::new(),
        };
        self.session_storage.insert_or_update(&session);
        return session;
    }
}
#[async_trait::async_trait]
impl SessionManager<DefaultSession> for DefaultSessionManager {
    async fn session_for_request(&mut self, req: &RequestCtx) -> DefaultSession {
        let cookie = req.cookies.get("session_id");
        if cookie.is_some() {
            let cookie = cookie.unwrap();
            let session_id = cookie.value();
            let session = self.session_storage.get(session_id.to_string()).await;
            if session.is_some() {
                let session = session.unwrap();
                return session;
            }
        }
        return self.generate_new_session(req).await;
    }
    async fn generate_session_id(&self, req: &RequestCtx) -> String {
        let session_id = Uuid::new_v4().to_string();
        session_id
    }

    async fn save_session(&mut self, req: &mut RequestCtx) {
        req.session.new = false;
        self.session_storage.insert_or_update(&req.session).await;
    }
}

pub enum SessionError {}
