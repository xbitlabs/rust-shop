use std::borrow::{Borrow, BorrowMut};
use std::convert::Infallible;
use std::error::Error;
use std::fs;
use std::io::{Read, Write};

use chrono::{Local, Utc};
use hyper::body::Buf;
use hyper::header::CONTENT_TYPE;
use hyper::{header, Body, Request, Response, StatusCode};
use log::{debug, error, info, warn};
use log4rs;
use multer::Multipart;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use rust_shop_core::EndpointResultCode;

use crate::config::load_config::APP_CONFIG;
use crate::{EndpointResult, RequestCtx, ResponseBuilder};

pub struct UploadController;

impl UploadController {
    pub async fn upload(ctx: RequestCtx) -> anyhow::Result<Response<Body>> {
        let upload_result = UploadController::handle(ctx.request).await;
        match upload_result {
            Ok(result) => Ok(result),
            Err(e) => {
                let endpoint_result: EndpointResult<String> =
                    EndpointResult::server_error("上传失败");
                Ok(ResponseBuilder::with_endpoint_result(endpoint_result))
            }
        }
    }
    // A handler for incoming requests.
    async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
        // Extract the `multipart/form-data` boundary from the headers.
        let boundary = req
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|ct| ct.to_str().ok())
            .and_then(|ct| multer::parse_boundary(ct).ok());

        // Send `BAD_REQUEST` status if the content-type is not multipart/form-data.
        if boundary.is_none() {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("BAD REQUEST"))
                .unwrap());
        }

        // Process the multipart e.g. you can store them in files.
        let upload_result =
            UploadController::process_multipart(req.into_body(), boundary.unwrap()).await;
        match upload_result {
            Ok(result) => Ok(ResponseBuilder::with_endpoint_result(result)),
            Err(error) => Ok(ResponseBuilder::with_msg(
                "上传失败：内部服务器错误",
                EndpointResultCode::ServerError,
            )),
        }
    }

    // Process the request body as multipart/form-data.
    async fn process_multipart(
        body: Body,
        boundary: String,
    ) -> multer::Result<EndpointResult<String>> {
        // Create a Multipart instance from the request body.
        let mut multipart = Multipart::new(body, boundary);
        // Iterate over the fields, `next_field` method will return the next field if
        // available.
        while let Some(mut field) = multipart.next_field().await? {
            // Get the field name.
            let name = field.name();

            // Get the field's filename if provided in "Content-Disposition" header.
            let mut file_name = field.file_name();
            let is_file = file_name.is_some() && !file_name.unwrap().is_empty();

            // Get the "Content-Type" header as `mime::Mime` type.
            let content_type = field.content_type();

            println!(
                "Name: {:?}, FileName: {:?}, Content-Type: {:?}",
                name, file_name, content_type
            );

            // Process the field data chunks e.g. store them in a file.
            let mut field_bytes_len = 0;
            if is_file {
                //文件后缀
                let mut filename_extension = "".to_string();
                let upload_file_name = file_name.unwrap();
                if upload_file_name.contains(".") {
                    let tmp: Vec<&str> = upload_file_name.split(".").collect();
                    filename_extension = ".".to_string() + tmp[1];
                }

                let upload_config = &APP_CONFIG.upload;
                //上传保存路径
                let mut save_as = String::from(upload_config.save_path.as_str());
                //路径分隔符
                let mut path_separator = "/";
                if cfg!(target_os = "windows") {
                    path_separator = "\\";
                    save_as = save_as.replace("/", "\\");
                } else {
                    save_as = save_as.replace("\\", "/");
                }
                //文件保存的根目录
                //save_as.push_str(upload_config.save_path.as_str());
                //如果配置没有以分隔符结尾，需要加上
                if !upload_config.save_path.ends_with(path_separator) {
                    save_as.push_str(path_separator);
                }
                //当前日期，每天上传的文件按天按目录保存
                let now = Local::now();
                let date = now.format("%Y%m%d").to_string();
                //保存路径加上日期
                save_as.push_str(date.as_str());
                //如果日期目录不存在就创建
                let path = std::path::Path::new(&save_as);
                if !path.exists() {
                    info!("创建上传文件夹{}", &save_as);
                    let mut is_create_dir_success = true;
                    let create_dir_result = fs::create_dir_all(&save_as);
                    match create_dir_result {
                        Ok(_) => {}
                        Err(e) => {
                            error!("创建上传文件夹失败：{}", e);
                            let endpoint_result: EndpointResult<String> =
                                EndpointResult::server_error("上传失败");
                            return Ok(endpoint_result);
                        }
                    }
                } else {
                    debug!("无需创建上传文件夹");
                }
                //文件夹分隔符
                save_as.push_str(path_separator);
                //guid文件名
                let id = Uuid::new_v4();
                let file_id = id.to_string();
                save_as.push_str(&*file_id);
                //文件后缀
                save_as.push_str(filename_extension.as_str());
                let mut new_file = fs::File::create(save_as);
                return match new_file {
                    Ok(mut file) => {
                        while let Some(field_chunk) = field.chunk().await? {
                            // Do something with field chunk.
                            field_bytes_len += field_chunk.len();
                            let write_result = file.write_all(&*field_chunk);
                            match write_result {
                                Ok(_) => {}
                                Err(e) => {
                                    error!("写入上传文件失败：{}", e);
                                    let endpoint_result: EndpointResult<String> =
                                        EndpointResult::server_error("上传失败");
                                    return Ok(endpoint_result);
                                }
                            }
                        }
                        let static_file_config = &APP_CONFIG.static_file;
                        let mut result_path: String = String::new();
                        if !static_file_config.virtual_path.starts_with("/") {
                            result_path.push('/');
                        }
                        result_path = result_path + static_file_config.virtual_path.as_str();
                        if !static_file_config.virtual_path.ends_with("/") {
                            result_path.push('/');
                        }
                        result_path = result_path
                            + date.as_str()
                            + "/"
                            + &*file_id
                            + filename_extension.as_str();
                        let endpoint_result =
                            EndpointResult::ok_with_payload("上传成功", result_path);
                        Ok(endpoint_result)
                    }
                    Err(e) => {
                        error!("创建文件异常：{}", e);
                        let endpoint_result = EndpointResult::server_error("上传失败");
                        Ok(endpoint_result)
                    }
                };
            } else {
                let name = name.unwrap().to_owned();
                let text = field.text().await?;
                println!("key={},val={}", name, text);
            }

            println!("Field Bytes Length: {:?}", field_bytes_len);
        }

        let endpoint_result = EndpointResult::client_error("请上传文件");
        return Ok(endpoint_result);
    }
}
