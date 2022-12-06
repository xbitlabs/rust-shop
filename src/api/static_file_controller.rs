use std::path::PathBuf;
use std::sync::Arc;

use hyper::{Body, Request, StatusCode, Uri};

use hyper_staticfile::Static;
use rust_shop_core::response::into_response::IntoResponse;
use rust_shop_core::response::Response;

use crate::config::load_config::APP_CONFIG;
use crate::{RequestCtx, ResponseBuilder};

pub struct StaticFileController;

impl StaticFileController {
    pub async fn handle(req: Request<Body>) -> anyhow::Result<Response> {
        let upload_config = &APP_CONFIG.upload;
        let mut static_ = Static::new(upload_config.save_path.as_str());
        static_.custom_path_resolver = Some(Arc::new(custom_path_resolver));
        let response_future = static_.serve(req);
        let response_result = response_future.await;
        match response_result {
            Ok(response) => Ok(response.into_response()),
            Err(_) => Ok(ResponseBuilder::with_status(
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }
}

pub fn custom_path_resolver(root_path: &str, request_uri: &str) -> PathBuf {
    let paths = request_uri.split_at(*&APP_CONFIG.static_file.virtual_path.len());
    let mut file_path: String = String::new();
    if cfg!(target_os = "windows") {
        file_path = file_path + root_path.replace("/", "\\").as_str();
    } else {
        file_path = file_path + root_path.replace("\\", "/").as_str();
    }
    file_path = file_path + paths.1;
    PathBuf::from(file_path)
}
