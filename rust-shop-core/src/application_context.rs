use crate::session::RedisSession;
use crate::DefaultSessionManager;
use once_cell::sync::Lazy;

pub static mut APPLICATION_CONTEXT: Lazy<ApplicationContext> = Lazy::new(|| {
    let mut application_context: ApplicationContext = ApplicationContext {
        session_manager: DefaultSessionManager::new(Box::new(RedisSession)),
    };
    application_context
});

pub struct ApplicationContext {
    pub session_manager: DefaultSessionManager,
}
