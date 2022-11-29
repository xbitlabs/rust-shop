use crate::RequestCtx;
use anyhow::anyhow;
use log::error;
use redis::Commands;
use serde::ser::{SerializeMap, SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Error;
use std::any::{Any, TypeId};
use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, LockResult, Mutex};
use uuid::Uuid;

pub trait Session {
    fn get_session_id(&self) -> &String;
    fn set_session_id(&mut self, session_id: String);
    fn is_new(&self) -> bool;
    fn set_new(&mut self, new: bool);
    fn insert<T>(&mut self, key: String, value: T)
    where
        T: 'static + serde::Serialize + for<'a> serde::Deserialize<'a> + Send + Sync;
    fn get<T>(&self, key: String) -> Option<T>
    where
        T: 'static + serde::Serialize + for<'a> serde::Deserialize<'a> + Send + Sync;
    fn remove<T: 'static + Sync + Send>(&mut self, key: String) -> Option<T>
    where
        T: 'static + serde::Serialize + for<'a> serde::Deserialize<'a> + Send + Sync;
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct DefaultSession {
    new: bool,
    session_id: String,
    inner: HashMap<String, String>,
}
impl Session for DefaultSession {
    fn get_session_id(&self) -> &String {
        &self.session_id
    }

    fn set_session_id(&mut self, session_id: String) {
        self.session_id = session_id;
    }

    fn is_new(&self) -> bool {
        self.new
    }
    fn set_new(&mut self, new: bool) {
        self.new = new;
    }
    fn insert<T>(&mut self, key: String, value: T)
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

pub trait SessionManager<T: Session> {
    fn session_for_request(&mut self, req: &RequestCtx) -> anyhow::Result<T>;
    fn generate_session_id(&self, req: &RequestCtx) -> String;
}
pub struct DefaultSessionManager {
    inner: HashMap<String, String>,
}
impl SessionManager<DefaultSession> for DefaultSessionManager {
    fn session_for_request(&mut self, req: &RequestCtx) -> anyhow::Result<DefaultSession> {
        let session_id = req.headers.get("session_id");
        if session_id.is_some() {
            let session_id = session_id.unwrap();
            let session_id = session_id.to_str();
            if session_id.is_ok() {
                let session_id = session_id.unwrap();
                if !session_id.is_empty() {
                    let session = self.inner.get(session_id);
                    if session.is_some() {
                        let session = session.unwrap();
                        let parse: Result<DefaultSession, Error> =
                            serde_json::from_str(session.as_str());
                        return match parse {
                            Ok(obj) => Ok(obj),
                            Err(err) => Err(anyhow!("获取session时转换json失败：{}", err)),
                        };
                    }
                }
            }
        }

        let new_session_id = self.generate_session_id(req);
        let session = DefaultSession {
            new: true,
            session_id: new_session_id.clone(),
            inner: HashMap::new(),
        };
        let session_to_json = serde_json::to_string(&session);
        self.inner
            .insert(new_session_id.clone(), session_to_json.unwrap());

        return Ok(session);
    }
    fn generate_session_id(&self, req: &RequestCtx) -> String {
        let session_id = Uuid::new_v4().to_string();
        session_id
    }
}

pub enum SessionError {}
