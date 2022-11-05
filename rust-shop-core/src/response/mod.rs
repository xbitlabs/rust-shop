use hyper::http;

mod into_response;

pub type Response<T = BoxBody> = http::Response<T>;
