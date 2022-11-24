use moka::sync::Cache;
use once_cell::sync::Lazy;
use std::any::Any;
use std::sync::Arc;

pub struct CacheEntity {
    value: Arc<dyn Any + Send + Sync>,
}
impl CacheEntity {
    pub fn into<T: Send + Sync + 'static>(&self) -> Option<&T> {
        let result: Option<&T> = self.value.downcast_ref();
        if result.is_some() {
            Some(result.unwrap())
        } else {
            None
        }
    }
}
impl Clone for CacheEntity {
    fn clone(&self) -> Self {
        CacheEntity {
            value: self.value.clone(),
        }
    }
}
pub static mut CACHE: Lazy<Cache<String, CacheEntity>> = Lazy::new(|| {
    let mut cache = Cache::builder()
        // Up to 10,000 entries.
        .max_capacity(10_000)
        // Create the cache.
        .build();
    cache
});
