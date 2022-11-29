use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::{DefaultSessionManager, Router};
use crate::session::RedisSession;

pub static mut APPLICATION_CONTEXT: Lazy<ApplicationContext> = Lazy::new(|| {
    let mut application_context: ApplicationContext = ApplicationContext{
        session_manager: DefaultSessionManager::new(Box::new(RedisSession))
    };
    application_context
});

pub struct ApplicationContext{
    pub session_manager:DefaultSessionManager
}