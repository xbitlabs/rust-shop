use crate::extract::{ExtractError, FromRequest};
use crate::RequestCtx;
use async_trait::async_trait;
use cookie::{Cookie, Key, PrivateJar};
use http::header::COOKIE;

use std::fmt;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct CookieJar {
    jar: cookie::CookieJar,
}

impl Default for CookieJar {
    fn default() -> Self {
        CookieJar {
            jar: Default::default(),
        }
    }
}

impl CookieJar {
    pub fn get(&self, name: &str) -> Option<&Cookie<'static>> {
        self.jar.get(name)
    }

    #[must_use]
    pub fn remove(mut self, cookie: Cookie<'static>) -> Self {
        self.jar.remove(cookie);
        self
    }

    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, cookie: Cookie<'static>) -> Self {
        self.jar.add(cookie);
        self
    }
    pub fn iter(&self) -> impl Iterator<Item = &Cookie<'static>> + '_ {
        CookieJarIter {
            jar: self,
            iter: self.jar.iter(),
        }
    }
}
struct CookieJarIter<'a> {
    jar: &'a CookieJar,
    iter: cookie::Iter<'a>,
}

impl<'a> Iterator for CookieJarIter<'a> {
    type Item = &'a Cookie<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let cookie = self.iter.next()?;

            if let Some(cookie) = self.jar.get(cookie.name()) {
                return Some(cookie);
            }
        }
    }
}
#[async_trait]
impl FromRequest for CookieJar {
    type Rejection = ExtractError;

    async fn from_request(req: &mut RequestCtx) -> Result<Self, Self::Rejection> {
        let mut jar = cookie::CookieJar::new();
        for cookie in cookies_from_request(req) {
            jar.add_original(cookie);
        }
        Ok(Self { jar })
    }
}
pub struct PrivateCookieJar<K = Key> {
    jar: cookie::CookieJar,
    key: Key,
    // The key used to extract the key extension. Allows users to use multiple keys for different
    // jars. Maybe a library wants its own key.
    _marker: PhantomData<K>,
}

impl<K> fmt::Debug for PrivateCookieJar<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrivateCookieJar")
            .field("jar", &self.jar)
            .field("key", &"REDACTED")
            .finish()
    }
}

#[async_trait]
impl<K> FromRequest for PrivateCookieJar<K>
where
    K: Into<Key> + Clone + Send + Sync + 'static,
{
    type Rejection = ExtractError;

    async fn from_request(req: &mut RequestCtx) -> anyhow::Result<Self, Self::Rejection> {
        let key = Key::from(b"");
        let mut jar = cookie::CookieJar::new();
        let mut private_jar = jar.private_mut(&key);
        for cookie in cookies_from_request(req) {
            if let Some(cookie) = private_jar.decrypt(cookie) {
                private_jar.add_original(cookie);
            }
        }

        Ok(Self {
            jar,
            key,
            _marker: PhantomData,
        })
    }
}

impl<K> PrivateCookieJar<K> {
    pub fn get(&self, name: &str) -> Option<Cookie<'static>> {
        self.private_jar().get(name)
    }

    #[must_use]
    pub fn remove(mut self, cookie: Cookie<'static>) -> Self {
        self.private_jar_mut().remove(cookie);
        self
    }

    #[must_use]
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, cookie: Cookie<'static>) -> Self {
        self.private_jar_mut().add(cookie);
        self
    }

    pub fn iter(&self) -> impl Iterator<Item = Cookie<'static>> + '_ {
        PrivateCookieJarIter {
            jar: self,
            iter: self.jar.iter(),
        }
    }

    fn private_jar(&self) -> PrivateJar<&'_ cookie::CookieJar> {
        self.jar.private(&self.key)
    }

    fn private_jar_mut(&mut self) -> PrivateJar<&'_ mut cookie::CookieJar> {
        self.jar.private_mut(&self.key)
    }
}

struct PrivateCookieJarIter<'a, K> {
    jar: &'a PrivateCookieJar<K>,
    iter: cookie::Iter<'a>,
}

impl<'a, K> Iterator for PrivateCookieJarIter<'a, K> {
    type Item = Cookie<'static>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let cookie = self.iter.next()?;

            if let Some(cookie) = self.jar.get(cookie.name()) {
                return Some(cookie);
            }
        }
    }
}
fn cookies_from_request(req: &mut RequestCtx) -> impl Iterator<Item = Cookie<'static>> + '_ {
    req.headers
        .get_all(COOKIE)
        .into_iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(';'))
        .filter_map(|cookie| Cookie::parse_encoded(cookie.to_owned()).ok())
}
