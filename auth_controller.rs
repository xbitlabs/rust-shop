#![feature(prelude_import)]
#![feature(try_trait_v2)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::any::Any;
use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::string::ToString;
use std::sync::{Arc, Mutex};
use hyper::{Body, Request, Response, StatusCode};
use lazy_static::lazy_static;
use log::info;
use once_cell::sync::Lazy;
use snowflake::SnowflakeIdGenerator;
use sqlx::{MySql, Pool};
use syn::__private::ToTokens;
use syn::{Item, ItemMod};
use rust_shop_core::db::{mysql_connection_pool, SqlCommandExecutor};
use rust_shop_core::extensions::Extensions;
use rust_shop_core::extract::json::Json;
use rust_shop_core::extract::{FromRequest, IntoResponse};
use rust_shop_core::router::register_route;
use rust_shop_core::security::NopPasswordEncoder;
use rust_shop_core::security::{
    AuthenticationTokenResolver, AuthenticationTokenResolverFn, DefaultLoadUserService,
    LoadUserService, LoadUserServiceFn, SecurityConfig,
    WeChatMiniAppAuthenticationTokenResolver, WeChatUserService,
};
use rust_shop_core::state::State;
use rust_shop_core::{
    AccessLogFilter, EndpointResult, Filter, Next, RequestCtx, RequestStateProvider,
    ResponseBuilder, Server,
};
use crate::api::auth_controller;
use crate::api::index_controller::IndexController;
use crate::config::load_config::APP_CONFIG;
pub mod api {
    pub(crate) mod auth_controller {
        pub mod AuthController {
            use std::any::Any;
            use std::borrow::BorrowMut;
            use std::convert::Infallible;
            use std::ops::Deref;
            use std::string::ToString;
            use std::sync::Arc;
            use std::thread;
            use anyhow::anyhow;
            use chrono::Local;
            use hyper::Body;
            use hyper::Response;
            use lazy_static::lazy_static;
            use sqlx::{Arguments, MySql, Pool, Row};
            use uuid::Uuid;
            use rust_shop_core::extensions::Extensions;
            use rust_shop_core::extract::extension::Extension;
            use rust_shop_core::extract::form::Form;
            use rust_shop_core::extract::header::Header;
            use rust_shop_core::extract::json::Json;
            use rust_shop_core::extract::path_variable::PathVariable;
            use rust_shop_core::extract::query::Query;
            use rust_shop_core::extract::request_param::RequestParam;
            use rust_shop_core::extract::request_state::RequestState;
            use rust_shop_core::extract::FromRequest;
            use rust_shop_core::extract::IntoResponse;
            use rust_shop_core::id_generator::ID_GENERATOR;
            use rust_shop_core::router::register_route;
            use rust_shop_core::security::UserDetails;
            use rust_shop_core::state::State;
            use rust_shop_core::RequestCtx;
            use rust_shop_core::{EndpointResult, RequestStateResolver, ResponseBuilder};
            use rust_shop_core::db::SqlCommandExecutor;
            use rust_shop_macro::route;
            use crate::entity::entity::ProductCategory;
            use crate::service::product_category_service::ProductCategoryService;
            use crate::StatusCode;
            use rust_shop_core::db::TransactionManager;
            use rust_shop_core::APP_EXTENSIONS;
            pub struct User {
                pub id: u32,
                pub name: String,
            }
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for User {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = match _serde::Serializer::serialize_struct(
                            __serializer,
                            "User",
                            false as usize + 1 + 1,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "id",
                            &self.id,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        match _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "name",
                            &self.name,
                        ) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        };
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for User {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        struct __FieldVisitor;
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private::Ok(__Field::__field0),
                                    1u64 => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "id" => _serde::__private::Ok(__Field::__field0),
                                    "name" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"id" => _serde::__private::Ok(__Field::__field0),
                                    b"name" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        struct __Visitor<'de> {
                            marker: _serde::__private::PhantomData<User>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = User;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct User",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match match _serde::de::SeqAccess::next_element::<
                                    u32,
                                >(&mut __seq) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct User with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match match _serde::de::SeqAccess::next_element::<
                                    String,
                                >(&mut __seq) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct User with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(User {
                                    id: __field0,
                                    name: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private::Option<u32> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                                while let _serde::__private::Some(__key)
                                    = match _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    } {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                match _serde::de::MapAccess::next_value::<
                                                    String,
                                                >(&mut __map) {
                                                    _serde::__private::Ok(__val) => __val,
                                                    _serde::__private::Err(__err) => {
                                                        return _serde::__private::Err(__err);
                                                    }
                                                },
                                            );
                                        }
                                        _ => {
                                            let _ = match _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            };
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private::Some(__field0) => __field0,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("id") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        match _serde::__private::de::missing_field("name") {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        }
                                    }
                                };
                                _serde::__private::Ok(User {
                                    id: __field0,
                                    name: __field1,
                                })
                            }
                        }
                        const FIELDS: &'static [&'static str] = &["id", "name"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "User",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<User>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::fmt::Debug for User {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "User",
                        "id",
                        &&self.id,
                        "name",
                        &&self.name,
                    )
                }
            }
            pub async fn test(
                Header(token): Header<Option<String>>,
                Header(cookie): Header<String>,
                PathVariable(id): PathVariable<Option<u32>>,
                PathVariable(age): PathVariable<u32>,
                RequestParam(name): RequestParam<Option<String>>,
                RequestParam(address): RequestParam<String>,
                Form(user): Form<User>,
                Query(user1): Query<User>,
                sql_exe_with_tran: &mut SqlCommandExecutor<'_, '_>,
            ) -> anyhow::Result<Json<User>> {
                let u = User {
                    id: id.unwrap(),
                    name: name.unwrap(),
                };
                Ok(Json(u))
            }
            pub async fn test_handler_proxy(
                mut req_ctx: RequestCtx,
            ) -> anyhow::Result<Response<Body>> {
                let ctx = &mut req_ctx;
                let mut pool_state: Option<&State<Pool<MySql>>> = None;
                unsafe {
                    pool_state = APP_EXTENSIONS.get();
                }
                let pool = pool_state.unwrap().get_ref();
                let tran = pool.begin().await?;
                let mut tran_manager = TransactionManager::new(tran);
                let mut sql_exe_with_tran = SqlCommandExecutor::WithTransaction(
                    &mut tran_manager,
                );
                let mut token: Header<Option<String>> = Header(None);
                let token_tmp_var = ctx.headers.get("token");
                if token_tmp_var.is_some() {
                    let token_tmp_var = token_tmp_var.unwrap();
                    if token_tmp_var.is_some() {
                        token = Header(
                            Some(token_tmp_var.as_ref().unwrap().to_string()),
                        );
                    }
                }
                let mut cookie_tmp_var_1: Option<Header<String>> = None;
                let cookie_tmp_var_2 = ctx.headers.get("cookie");
                if cookie_tmp_var_2.is_none() {
                    return Err(
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                ::core::fmt::Arguments::new_v1(
                                    &["header \'cookie\' is None"],
                                    &[],
                                ),
                            );
                            error
                        }),
                    );
                } else {
                    let cookie_tmp_var_2 = cookie_tmp_var_2.unwrap();
                    if cookie_tmp_var_2.is_none() {
                        return Err(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["header \'cookie\' is None"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        );
                    } else {
                        cookie_tmp_var_1 = Some(
                            Header(cookie_tmp_var_2.as_ref().unwrap().to_string()),
                        );
                    }
                }
                let cookie: Header<String> = cookie_tmp_var_1.unwrap();
                let mut id: PathVariable<Option<u32>> = PathVariable(None);
                let id_tmp_var = ctx.router_params.find("id");
                if id_tmp_var.is_some() {
                    let id_tmp_var = id_tmp_var.unwrap().to_string();
                    let id_tmp_var = id_tmp_var.parse::<u32>();
                    if id_tmp_var.is_err() {
                        return Err(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["PathVariable \'id\' is invalid"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        );
                    } else {
                        id = PathVariable(Some(id_tmp_var.unwrap()));
                    }
                }
                let mut age: Option<PathVariable<u32>> = None;
                let age_tmp_var = ctx.router_params.find("age");
                if age_tmp_var.is_none() {
                    return Err(
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                ::core::fmt::Arguments::new_v1(
                                    &["router param \'age\' is None"],
                                    &[],
                                ),
                            );
                            error
                        }),
                    );
                } else {
                    let parse_result = age_tmp_var.unwrap().to_string().parse::<u32>();
                    if parse_result.is_err() {
                        return Err(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["router param \'age\' is invalid"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        );
                    } else {
                        age = Some(PathVariable(parse_result.unwrap()));
                    }
                }
                let age = age.unwrap();
                let mut name: RequestParam<Option<String>> = RequestParam(None);
                let name_tmp_var = ctx.query_params.get("name");
                if name_tmp_var.is_some() {
                    let name_tmp_var = name_tmp_var.unwrap().to_string();
                    let name_tmp_var = name_tmp_var.parse::<String>();
                    if name_tmp_var.is_err() {
                        return Err(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["RequestParam \'name\' is invalid"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        );
                    } else {
                        name = RequestParam(Some(name_tmp_var.unwrap()));
                    }
                }
                let mut address: Option<RequestParam<String>> = None;
                let address_tmp_var = ctx.query_params.get("address");
                if address_tmp_var.is_none() {
                    return Err(
                        ::anyhow::__private::must_use({
                            let error = ::anyhow::__private::format_err(
                                ::core::fmt::Arguments::new_v1(
                                    &["router param \'address\' is None"],
                                    &[],
                                ),
                            );
                            error
                        }),
                    );
                } else {
                    let parse_result = address_tmp_var
                        .unwrap()
                        .to_string()
                        .parse::<String>();
                    if parse_result.is_err() {
                        return Err(
                            ::anyhow::__private::must_use({
                                let error = ::anyhow::__private::format_err(
                                    ::core::fmt::Arguments::new_v1(
                                        &["router param \'address\' is invalid"],
                                        &[],
                                    ),
                                );
                                error
                            }),
                        );
                    } else {
                        address = Some(RequestParam(parse_result.unwrap()));
                    }
                }
                let address = address.unwrap();
                let user = Form::from_request(ctx).await?;
                let user1 = Query::from_request(ctx).await?;
                let handler_result = test(
                        token,
                        cookie,
                        id,
                        age,
                        name,
                        address,
                        user,
                        user1,
                        &mut sql_exe_with_tran,
                    )
                    .await;
                return if handler_result.is_err() {
                    {
                        {
                            ::std::io::_print(
                                ::core::fmt::Arguments::new_v1(
                                    &["", "\n"],
                                    &[::core::fmt::ArgumentV1::new_display(&"鎻愪氦浜嬪姟")],
                                ),
                            );
                        };
                        tran_manager.rollback().await?;
                        Err(handler_result.err().unwrap())
                    }
                } else {
                    {
                        {
                            ::std::io::_print(
                                ::core::fmt::Arguments::new_v1(
                                    &["", "\n"],
                                    &[::core::fmt::ArgumentV1::new_display(&"鍥炴粴浜嬪姟")],
                                ),
                            );
                        };
                        tran_manager.commit().await?;
                        Ok(handler_result.unwrap().into_response())
                    }
                };
            }
        }
    }
    pub(crate) mod category_controller {}
    pub(crate) mod index_controller {
        use std::any::Any;
        use anyhow::anyhow;
        use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
        use hyper::Body;
        use hyper::Request;
        use rust_shop_core::parse_form_params;
        use rust_shop_core::parse_request_json;
        use rust_shop_macro::FormParser;
        use crate::service::product_category_service::ProductCategoryService;
        use crate::{EndpointResult, RequestCtx, ResponseBuilder, StatusCode};
        pub struct Test {
            pub name: Option<String>,
            pub age: Option<u32>,
            pub address: Option<String>,
            pub time: Option<NaiveDateTime>,
            pub date: Option<NaiveDate>,
            pub a: Option<bool>,
            pub b: Option<i32>,
            pub c: Option<u32>,
            pub d: Option<i64>,
            pub e: Option<u64>,
            pub f: Option<f64>,
            pub h: Option<f32>,
            pub i: Option<isize>,
            pub j: Option<usize>,
            pub k: Option<i8>,
            pub l: Option<u8>,
            pub m: Option<i16>,
            pub n: Option<u16>,
            pub o: Option<i128>,
            pub p: Option<u128>,
        }
        impl Test {
            pub fn build_form_parser() -> TestFormParser {
                TestFormParser::default()
            }
        }
        pub struct TestFormParser {
            name: Option<String>,
            age: Option<u32>,
            address: Option<String>,
            time: Option<NaiveDateTime>,
            date: Option<NaiveDate>,
            a: Option<bool>,
            b: Option<i32>,
            c: Option<u32>,
            d: Option<i64>,
            e: Option<u64>,
            f: Option<f64>,
            h: Option<f32>,
            i: Option<isize>,
            j: Option<usize>,
            k: Option<i8>,
            l: Option<u8>,
            m: Option<i16>,
            n: Option<u16>,
            o: Option<i128>,
            p: Option<u128>,
        }
        #[automatically_derived]
        impl ::core::default::Default for TestFormParser {
            #[inline]
            fn default() -> TestFormParser {
                TestFormParser {
                    name: ::core::default::Default::default(),
                    age: ::core::default::Default::default(),
                    address: ::core::default::Default::default(),
                    time: ::core::default::Default::default(),
                    date: ::core::default::Default::default(),
                    a: ::core::default::Default::default(),
                    b: ::core::default::Default::default(),
                    c: ::core::default::Default::default(),
                    d: ::core::default::Default::default(),
                    e: ::core::default::Default::default(),
                    f: ::core::default::Default::default(),
                    h: ::core::default::Default::default(),
                    i: ::core::default::Default::default(),
                    j: ::core::default::Default::default(),
                    k: ::core::default::Default::default(),
                    l: ::core::default::Default::default(),
                    m: ::core::default::Default::default(),
                    n: ::core::default::Default::default(),
                    o: ::core::default::Default::default(),
                    p: ::core::default::Default::default(),
                }
            }
        }
        impl TestFormParser {
            pub async fn parse(self, req: &mut RequestCtx) -> anyhow::Result<Test> {
                let form_params = parse_form_params(req).await;
                let param = form_params.get("name");
                let mut name = None;
                if param.is_some() {
                    name = Some(param.unwrap().to_string());
                }
                let param = form_params.get("age");
                let mut age = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<u32>();
                    if parse_result.is_ok() {
                        age = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("address");
                let mut address = None;
                if param.is_some() {
                    address = Some(param.unwrap().to_string());
                }
                let param = form_params.get("time");
                let mut time = None;
                if param.is_some() {
                    let parse_result = NaiveDateTime::parse_from_str(
                        param.unwrap(),
                        "%Y-%m-%d %H:%M:%S",
                    );
                    if parse_result.is_ok() {
                        time = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("date");
                let mut date = None;
                if param.is_some() {
                    let parse_result = NaiveDate::parse_from_str(
                        param.unwrap(),
                        "%Y-%m-%d",
                    );
                    if parse_result.is_ok() {
                        date = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("a");
                let mut a = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<bool>();
                    if parse_result.is_ok() {
                        a = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("b");
                let mut b = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<i32>();
                    if parse_result.is_ok() {
                        b = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("c");
                let mut c = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<u32>();
                    if parse_result.is_ok() {
                        c = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("d");
                let mut d = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<i64>();
                    if parse_result.is_ok() {
                        d = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("e");
                let mut e = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<u64>();
                    if parse_result.is_ok() {
                        e = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("f");
                let mut f = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<f64>();
                    if parse_result.is_ok() {
                        f = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("h");
                let mut h = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<f32>();
                    if parse_result.is_ok() {
                        h = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("i");
                let mut i = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<isize>();
                    if parse_result.is_ok() {
                        i = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("j");
                let mut j = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<usize>();
                    if parse_result.is_ok() {
                        j = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("k");
                let mut k = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<i8>();
                    if parse_result.is_ok() {
                        k = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("l");
                let mut l = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<u8>();
                    if parse_result.is_ok() {
                        l = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("m");
                let mut m = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<i16>();
                    if parse_result.is_ok() {
                        m = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("n");
                let mut n = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<u16>();
                    if parse_result.is_ok() {
                        n = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("o");
                let mut o = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<i128>();
                    if parse_result.is_ok() {
                        o = Some(parse_result.unwrap());
                    }
                }
                let param = form_params.get("p");
                let mut p = None;
                if param.is_some() {
                    let parse_result = param.unwrap().parse::<u128>();
                    if parse_result.is_ok() {
                        p = Some(parse_result.unwrap());
                    }
                }
                Ok(Test {
                    name,
                    age,
                    address,
                    time,
                    date,
                    a,
                    b,
                    c,
                    d,
                    e,
                    f,
                    h,
                    i,
                    j,
                    k,
                    l,
                    m,
                    n,
                    o,
                    p,
                })
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Test {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __field10,
                        __field11,
                        __field12,
                        __field13,
                        __field14,
                        __field15,
                        __field16,
                        __field17,
                        __field18,
                        __field19,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                10u64 => _serde::__private::Ok(__Field::__field10),
                                11u64 => _serde::__private::Ok(__Field::__field11),
                                12u64 => _serde::__private::Ok(__Field::__field12),
                                13u64 => _serde::__private::Ok(__Field::__field13),
                                14u64 => _serde::__private::Ok(__Field::__field14),
                                15u64 => _serde::__private::Ok(__Field::__field15),
                                16u64 => _serde::__private::Ok(__Field::__field16),
                                17u64 => _serde::__private::Ok(__Field::__field17),
                                18u64 => _serde::__private::Ok(__Field::__field18),
                                19u64 => _serde::__private::Ok(__Field::__field19),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "name" => _serde::__private::Ok(__Field::__field0),
                                "age" => _serde::__private::Ok(__Field::__field1),
                                "address" => _serde::__private::Ok(__Field::__field2),
                                "time" => _serde::__private::Ok(__Field::__field3),
                                "date" => _serde::__private::Ok(__Field::__field4),
                                "a" => _serde::__private::Ok(__Field::__field5),
                                "b" => _serde::__private::Ok(__Field::__field6),
                                "c" => _serde::__private::Ok(__Field::__field7),
                                "d" => _serde::__private::Ok(__Field::__field8),
                                "e" => _serde::__private::Ok(__Field::__field9),
                                "f" => _serde::__private::Ok(__Field::__field10),
                                "h" => _serde::__private::Ok(__Field::__field11),
                                "i" => _serde::__private::Ok(__Field::__field12),
                                "j" => _serde::__private::Ok(__Field::__field13),
                                "k" => _serde::__private::Ok(__Field::__field14),
                                "l" => _serde::__private::Ok(__Field::__field15),
                                "m" => _serde::__private::Ok(__Field::__field16),
                                "n" => _serde::__private::Ok(__Field::__field17),
                                "o" => _serde::__private::Ok(__Field::__field18),
                                "p" => _serde::__private::Ok(__Field::__field19),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"name" => _serde::__private::Ok(__Field::__field0),
                                b"age" => _serde::__private::Ok(__Field::__field1),
                                b"address" => _serde::__private::Ok(__Field::__field2),
                                b"time" => _serde::__private::Ok(__Field::__field3),
                                b"date" => _serde::__private::Ok(__Field::__field4),
                                b"a" => _serde::__private::Ok(__Field::__field5),
                                b"b" => _serde::__private::Ok(__Field::__field6),
                                b"c" => _serde::__private::Ok(__Field::__field7),
                                b"d" => _serde::__private::Ok(__Field::__field8),
                                b"e" => _serde::__private::Ok(__Field::__field9),
                                b"f" => _serde::__private::Ok(__Field::__field10),
                                b"h" => _serde::__private::Ok(__Field::__field11),
                                b"i" => _serde::__private::Ok(__Field::__field12),
                                b"j" => _serde::__private::Ok(__Field::__field13),
                                b"k" => _serde::__private::Ok(__Field::__field14),
                                b"l" => _serde::__private::Ok(__Field::__field15),
                                b"m" => _serde::__private::Ok(__Field::__field16),
                                b"n" => _serde::__private::Ok(__Field::__field17),
                                b"o" => _serde::__private::Ok(__Field::__field18),
                                b"p" => _serde::__private::Ok(__Field::__field19),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Test>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Test;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Test",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<u32>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<NaiveDateTime>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                Option<NaiveDate>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match match _serde::de::SeqAccess::next_element::<
                                Option<i32>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match match _serde::de::SeqAccess::next_element::<
                                Option<u32>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match match _serde::de::SeqAccess::next_element::<
                                Option<i64>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match match _serde::de::SeqAccess::next_element::<
                                Option<u64>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field10 = match match _serde::de::SeqAccess::next_element::<
                                Option<f64>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            10usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field11 = match match _serde::de::SeqAccess::next_element::<
                                Option<f32>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            11usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field12 = match match _serde::de::SeqAccess::next_element::<
                                Option<isize>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            12usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field13 = match match _serde::de::SeqAccess::next_element::<
                                Option<usize>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            13usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field14 = match match _serde::de::SeqAccess::next_element::<
                                Option<i8>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            14usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field15 = match match _serde::de::SeqAccess::next_element::<
                                Option<u8>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            15usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field16 = match match _serde::de::SeqAccess::next_element::<
                                Option<i16>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            16usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field17 = match match _serde::de::SeqAccess::next_element::<
                                Option<u16>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            17usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field18 = match match _serde::de::SeqAccess::next_element::<
                                Option<i128>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            18usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            let __field19 = match match _serde::de::SeqAccess::next_element::<
                                Option<u128>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            19usize,
                                            &"struct Test with 20 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Test {
                                name: __field0,
                                age: __field1,
                                address: __field2,
                                time: __field3,
                                date: __field4,
                                a: __field5,
                                b: __field6,
                                c: __field7,
                                d: __field8,
                                e: __field9,
                                f: __field10,
                                h: __field11,
                                i: __field12,
                                j: __field13,
                                k: __field14,
                                l: __field15,
                                m: __field16,
                                n: __field17,
                                o: __field18,
                                p: __field19,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Option<u32>> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<NaiveDateTime>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<
                                Option<NaiveDate>,
                            > = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<Option<bool>> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<Option<i32>> = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<Option<u32>> = _serde::__private::None;
                            let mut __field8: _serde::__private::Option<Option<i64>> = _serde::__private::None;
                            let mut __field9: _serde::__private::Option<Option<u64>> = _serde::__private::None;
                            let mut __field10: _serde::__private::Option<Option<f64>> = _serde::__private::None;
                            let mut __field11: _serde::__private::Option<Option<f32>> = _serde::__private::None;
                            let mut __field12: _serde::__private::Option<
                                Option<isize>,
                            > = _serde::__private::None;
                            let mut __field13: _serde::__private::Option<
                                Option<usize>,
                            > = _serde::__private::None;
                            let mut __field14: _serde::__private::Option<Option<i8>> = _serde::__private::None;
                            let mut __field15: _serde::__private::Option<Option<u8>> = _serde::__private::None;
                            let mut __field16: _serde::__private::Option<Option<i16>> = _serde::__private::None;
                            let mut __field17: _serde::__private::Option<Option<u16>> = _serde::__private::None;
                            let mut __field18: _serde::__private::Option<Option<i128>> = _serde::__private::None;
                            let mut __field19: _serde::__private::Option<Option<u128>> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("age"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<u32>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "address",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("time"),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<NaiveDateTime>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("date"),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<NaiveDate>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("a"),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<bool>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("b"),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<i32>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("c"),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<u32>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("d"),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<i64>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("e"),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<u64>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field10 => {
                                        if _serde::__private::Option::is_some(&__field10) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("f"),
                                            );
                                        }
                                        __field10 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<f64>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field11 => {
                                        if _serde::__private::Option::is_some(&__field11) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("h"),
                                            );
                                        }
                                        __field11 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<f32>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field12 => {
                                        if _serde::__private::Option::is_some(&__field12) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("i"),
                                            );
                                        }
                                        __field12 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<isize>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field13 => {
                                        if _serde::__private::Option::is_some(&__field13) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("j"),
                                            );
                                        }
                                        __field13 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<usize>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field14 => {
                                        if _serde::__private::Option::is_some(&__field14) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("k"),
                                            );
                                        }
                                        __field14 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<i8>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field15 => {
                                        if _serde::__private::Option::is_some(&__field15) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("l"),
                                            );
                                        }
                                        __field15 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<u8>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field16 => {
                                        if _serde::__private::Option::is_some(&__field16) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("m"),
                                            );
                                        }
                                        __field16 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<i16>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field17 => {
                                        if _serde::__private::Option::is_some(&__field17) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("n"),
                                            );
                                        }
                                        __field17 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<u16>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field18 => {
                                        if _serde::__private::Option::is_some(&__field18) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("o"),
                                            );
                                        }
                                        __field18 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<i128>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field19 => {
                                        if _serde::__private::Option::is_some(&__field19) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("p"),
                                            );
                                        }
                                        __field19 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<u128>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("name") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("age") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("address") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("time") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("date") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("a") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("b") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("c") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("d") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("e") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field10 = match __field10 {
                                _serde::__private::Some(__field10) => __field10,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("f") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field11 = match __field11 {
                                _serde::__private::Some(__field11) => __field11,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("h") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field12 = match __field12 {
                                _serde::__private::Some(__field12) => __field12,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("i") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field13 = match __field13 {
                                _serde::__private::Some(__field13) => __field13,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("j") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field14 = match __field14 {
                                _serde::__private::Some(__field14) => __field14,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("k") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field15 = match __field15 {
                                _serde::__private::Some(__field15) => __field15,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("l") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field16 = match __field16 {
                                _serde::__private::Some(__field16) => __field16,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("m") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field17 = match __field17 {
                                _serde::__private::Some(__field17) => __field17,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("n") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field18 = match __field18 {
                                _serde::__private::Some(__field18) => __field18,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("o") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field19 = match __field19 {
                                _serde::__private::Some(__field19) => __field19,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("p") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Test {
                                name: __field0,
                                age: __field1,
                                address: __field2,
                                time: __field3,
                                date: __field4,
                                a: __field5,
                                b: __field6,
                                c: __field7,
                                d: __field8,
                                e: __field9,
                                f: __field10,
                                h: __field11,
                                i: __field12,
                                j: __field13,
                                k: __field14,
                                l: __field15,
                                m: __field16,
                                n: __field17,
                                o: __field18,
                                p: __field19,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "name",
                        "age",
                        "address",
                        "time",
                        "date",
                        "a",
                        "b",
                        "c",
                        "d",
                        "e",
                        "f",
                        "h",
                        "i",
                        "j",
                        "k",
                        "l",
                        "m",
                        "n",
                        "o",
                        "p",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Test",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Test>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Test {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Test",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                            + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "age",
                        &self.age,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "address",
                        &self.address,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "time",
                        &self.time,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "date",
                        &self.date,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "a",
                        &self.a,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "b",
                        &self.b,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "c",
                        &self.c,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "d",
                        &self.d,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "e",
                        &self.e,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "f",
                        &self.f,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "h",
                        &self.h,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "i",
                        &self.i,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "j",
                        &self.j,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "k",
                        &self.k,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "l",
                        &self.l,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "m",
                        &self.m,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "n",
                        &self.n,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "o",
                        &self.o,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "p",
                        &self.p,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for Test {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "name",
                    "age",
                    "address",
                    "time",
                    "date",
                    "a",
                    "b",
                    "c",
                    "d",
                    "e",
                    "f",
                    "h",
                    "i",
                    "j",
                    "k",
                    "l",
                    "m",
                    "n",
                    "o",
                    "p",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.name,
                    &&self.age,
                    &&self.address,
                    &&self.time,
                    &&self.date,
                    &&self.a,
                    &&self.b,
                    &&self.c,
                    &&self.d,
                    &&self.e,
                    &&self.f,
                    &&self.h,
                    &&self.i,
                    &&self.j,
                    &&self.k,
                    &&self.l,
                    &&self.m,
                    &&self.n,
                    &&self.o,
                    &&self.p,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "Test",
                    names,
                    values,
                )
            }
        }
        pub struct IndexController;
        impl<'a> IndexController {
            pub async fn index(
                mut ctx: RequestCtx,
            ) -> anyhow::Result<hyper::Response<hyper::Body>> {
                let endpoint_result = EndpointResult::ok_with_payload("", "");
                Ok(ResponseBuilder::with_endpoint_result(endpoint_result))
            }
        }
    }
    pub(crate) mod product_controller {}
    pub(crate) mod static_file_controller {}
    pub(crate) mod upload_controller {}
}
mod config {
    pub(crate) mod env_config {
        pub struct AppConfig {
            pub server: Server,
            pub upload: Upload,
            pub static_file: StaticFile,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for AppConfig {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "AppConfig",
                        false as usize + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "server",
                        &self.server,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "upload",
                        &self.upload,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "static_file",
                        &self.static_file,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for AppConfig {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "server" => _serde::__private::Ok(__Field::__field0),
                                "upload" => _serde::__private::Ok(__Field::__field1),
                                "static_file" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"server" => _serde::__private::Ok(__Field::__field0),
                                b"upload" => _serde::__private::Ok(__Field::__field1),
                                b"static_file" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<AppConfig>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = AppConfig;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct AppConfig",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Server,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct AppConfig with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Upload,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct AppConfig with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                StaticFile,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct AppConfig with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(AppConfig {
                                server: __field0,
                                upload: __field1,
                                static_file: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Server> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Upload> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<StaticFile> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("server"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Server,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("upload"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Upload,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "static_file",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                StaticFile,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("server") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("upload") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("static_file") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(AppConfig {
                                server: __field0,
                                upload: __field1,
                                static_file: __field2,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "server",
                        "upload",
                        "static_file",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "AppConfig",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<AppConfig>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Upload {
            pub save_path: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Upload {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Upload",
                    "save_path",
                    &&self.save_path,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Upload {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Upload",
                        false as usize + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "save_path",
                        &self.save_path,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Upload {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "save_path" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"save_path" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Upload>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Upload;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Upload",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Upload with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Upload { save_path: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "save_path",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("save_path") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Upload { save_path: __field0 })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["save_path"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Upload",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Upload>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct StaticFile {
            pub virtual_path: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for StaticFile {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "StaticFile",
                    "virtual_path",
                    &&self.virtual_path,
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for StaticFile {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "StaticFile",
                        false as usize + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "virtual_path",
                        &self.virtual_path,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for StaticFile {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "virtual_path" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"virtual_path" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<StaticFile>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = StaticFile;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct StaticFile",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct StaticFile with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(StaticFile {
                                virtual_path: __field0,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "virtual_path",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("virtual_path") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(StaticFile {
                                virtual_path: __field0,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["virtual_path"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "StaticFile",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<StaticFile>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        pub struct Server {
            pub port: u32,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Server {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Server",
                        false as usize + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "port",
                        &self.port,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for Server {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "port" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"port" => _serde::__private::Ok(__Field::__field0),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<Server>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = Server;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct Server",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                u32,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct Server with 1 element",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(Server { port: __field0 })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<u32> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("port"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("port") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(Server { port: __field0 })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["port"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "Server",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<Server>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
    }
    pub(crate) mod load_config {
        use std::fs::read_to_string;
        use lazy_static::lazy_static;
        use schemars::schema::RootSchema;
        use rust_shop_core::app_config::{load_conf, EnvConfig};
        use crate::config::env_config::AppConfig;
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        ///
        /// 鍏ㄥ眬閰嶇疆
        ///
        pub struct APP_CONFIG {
            __private_field: (),
        }
        #[doc(hidden)]
        pub static APP_CONFIG: APP_CONFIG = APP_CONFIG { __private_field: () };
        impl ::lazy_static::__Deref for APP_CONFIG {
            type Target = AppConfig;
            fn deref(&self) -> &AppConfig {
                #[inline(always)]
                fn __static_ref_initialize() -> AppConfig {
                    load_conf().unwrap()
                }
                #[inline(always)]
                fn __stability() -> &'static AppConfig {
                    static LAZY: ::lazy_static::lazy::Lazy<AppConfig> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for APP_CONFIG {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
    }
}
mod core {}
pub mod entity {
    pub(crate) mod entity {
        use chrono::NaiveDateTime;
        pub struct ProductCategory {
            pub id: i64,
            pub name: String,
            pub icon: Option<String>,
            pub pic: Option<String>,
            pub sort_index: i32,
        }
        #[automatically_derived]
        impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for ProductCategory
        where
            &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
            Option<String>: ::sqlx::types::Type<R::Database>,
            Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
            Option<String>: ::sqlx::types::Type<R::Database>,
            i32: ::sqlx::decode::Decode<'a, R::Database>,
            i32: ::sqlx::types::Type<R::Database>,
        {
            fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
                let id: i64 = row.try_get("id")?;
                let name: String = row.try_get("name")?;
                let icon: Option<String> = row.try_get("icon")?;
                let pic: Option<String> = row.try_get("pic")?;
                let sort_index: i32 = row.try_get("sort_index")?;
                ::std::result::Result::Ok(ProductCategory {
                    id,
                    name,
                    icon,
                    pic,
                    sort_index,
                })
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ProductCategory {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "ProductCategory",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "icon",
                        &self.icon,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "pic",
                        &self.pic,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "sort_index",
                        &self.sort_index,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for ProductCategory {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "name" => _serde::__private::Ok(__Field::__field1),
                                "icon" => _serde::__private::Ok(__Field::__field2),
                                "pic" => _serde::__private::Ok(__Field::__field3),
                                "sort_index" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"name" => _serde::__private::Ok(__Field::__field1),
                                b"icon" => _serde::__private::Ok(__Field::__field2),
                                b"pic" => _serde::__private::Ok(__Field::__field3),
                                b"sort_index" => _serde::__private::Ok(__Field::__field4),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<ProductCategory>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = ProductCategory;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct ProductCategory",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                i64,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct ProductCategory with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct ProductCategory with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct ProductCategory with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct ProductCategory with 5 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match match _serde::de::SeqAccess::next_element::<
                                i32,
                            >(&mut __seq) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct ProductCategory with 5 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(ProductCategory {
                                id: __field0,
                                name: __field1,
                                icon: __field2,
                                pic: __field3,
                                sort_index: __field4,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<i64> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<
                                Option<String>,
                            > = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<i32> = _serde::__private::None;
                            while let _serde::__private::Some(__key)
                                = match _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                } {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<i64>(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                String,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("icon"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("pic"),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<
                                                Option<String>,
                                            >(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "sort_index",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<i32>(&mut __map) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("id") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("name") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("icon") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("pic") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("sort_index") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(ProductCategory {
                                id: __field0,
                                name: __field1,
                                icon: __field2,
                                pic: __field3,
                                sort_index: __field4,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &[
                        "id",
                        "name",
                        "icon",
                        "pic",
                        "sort_index",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "ProductCategory",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<ProductCategory>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::fmt::Debug for ProductCategory {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "ProductCategory",
                    "id",
                    &&self.id,
                    "name",
                    &&self.name,
                    "icon",
                    &&self.icon,
                    "pic",
                    &&self.pic,
                    "sort_index",
                    &&self.sort_index,
                )
            }
        }
        pub struct Product {
            pub id: i64,
            pub name: String,
            pub cover_image: String,
            pub category_id: i64,
            pub pics_and_video: String,
            pub description: String,
            pub status: String,
            #[serde(with = "rust_shop_core::entity::db_numeric_date")]
            pub created_time: NaiveDateTime,
            #[serde(with = "rust_shop_core::entity::db_numeric_date")]
            pub last_modified_time: NaiveDateTime,
        }
        #[automatically_derived]
        impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for Product
        where
            &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            NaiveDateTime: ::sqlx::decode::Decode<'a, R::Database>,
            NaiveDateTime: ::sqlx::types::Type<R::Database>,
            NaiveDateTime: ::sqlx::decode::Decode<'a, R::Database>,
            NaiveDateTime: ::sqlx::types::Type<R::Database>,
        {
            fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
                let id: i64 = row.try_get("id")?;
                let name: String = row.try_get("name")?;
                let cover_image: String = row.try_get("cover_image")?;
                let category_id: i64 = row.try_get("category_id")?;
                let pics_and_video: String = row.try_get("pics_and_video")?;
                let description: String = row.try_get("description")?;
                let status: String = row.try_get("status")?;
                let created_time: NaiveDateTime = row.try_get("created_time")?;
                let last_modified_time: NaiveDateTime = row
                    .try_get("last_modified_time")?;
                ::std::result::Result::Ok(Product {
                    id,
                    name,
                    cover_image,
                    category_id,
                    pics_and_video,
                    description,
                    status,
                    created_time,
                    last_modified_time,
                })
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Product {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Product",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "cover_image",
                        &self.cover_image,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "category_id",
                        &self.category_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "pics_and_video",
                        &self.pics_and_video,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "description",
                        &self.description,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "status",
                        &self.status,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "created_time",
                        {
                            struct __SerializeWith<'__a> {
                                values: (&'__a NaiveDateTime,),
                                phantom: _serde::__private::PhantomData<Product>,
                            }
                            impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                fn serialize<__S>(
                                    &self,
                                    __s: __S,
                                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    rust_shop_core::entity::db_numeric_date::serialize(
                                        self.values.0,
                                        __s,
                                    )
                                }
                            }
                            &__SerializeWith {
                                values: (&self.created_time,),
                                phantom: _serde::__private::PhantomData::<Product>,
                            }
                        },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "last_modified_time",
                        {
                            struct __SerializeWith<'__a> {
                                values: (&'__a NaiveDateTime,),
                                phantom: _serde::__private::PhantomData<Product>,
                            }
                            impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                fn serialize<__S>(
                                    &self,
                                    __s: __S,
                                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    rust_shop_core::entity::db_numeric_date::serialize(
                                        self.values.0,
                                        __s,
                                    )
                                }
                            }
                            &__SerializeWith {
                                values: (&self.last_modified_time,),
                                phantom: _serde::__private::PhantomData::<Product>,
                            }
                        },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct Order {
            pub id: i64,
            pub user_id: i64,
            pub logistics_status: Option<String>,
            pub pay_status: String,
            pub recipient: String,
            pub phone_number: String,
            pub address: String,
            pub post_code: String,
            pub remark: Option<String>,
            #[serde(with = "rust_shop_core::entity::db_numeric_date")]
            pub created_time: NaiveDateTime,
        }
        #[automatically_derived]
        impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for Order
        where
            &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
            Option<String>: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
            Option<String>: ::sqlx::types::Type<R::Database>,
            NaiveDateTime: ::sqlx::decode::Decode<'a, R::Database>,
            NaiveDateTime: ::sqlx::types::Type<R::Database>,
        {
            fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
                let id: i64 = row.try_get("id")?;
                let user_id: i64 = row.try_get("user_id")?;
                let logistics_status: Option<String> = row.try_get("logistics_status")?;
                let pay_status: String = row.try_get("pay_status")?;
                let recipient: String = row.try_get("recipient")?;
                let phone_number: String = row.try_get("phone_number")?;
                let address: String = row.try_get("address")?;
                let post_code: String = row.try_get("post_code")?;
                let remark: Option<String> = row.try_get("remark")?;
                let created_time: NaiveDateTime = row.try_get("created_time")?;
                ::std::result::Result::Ok(Order {
                    id,
                    user_id,
                    logistics_status,
                    pay_status,
                    recipient,
                    phone_number,
                    address,
                    post_code,
                    remark,
                    created_time,
                })
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Order {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Order",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "user_id",
                        &self.user_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "logistics_status",
                        &self.logistics_status,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "pay_status",
                        &self.pay_status,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "recipient",
                        &self.recipient,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "phone_number",
                        &self.phone_number,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "address",
                        &self.address,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "post_code",
                        &self.post_code,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "remark",
                        &self.remark,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "created_time",
                        {
                            struct __SerializeWith<'__a> {
                                values: (&'__a NaiveDateTime,),
                                phantom: _serde::__private::PhantomData<Order>,
                            }
                            impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                fn serialize<__S>(
                                    &self,
                                    __s: __S,
                                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    rust_shop_core::entity::db_numeric_date::serialize(
                                        self.values.0,
                                        __s,
                                    )
                                }
                            }
                            &__SerializeWith {
                                values: (&self.created_time,),
                                phantom: _serde::__private::PhantomData::<Order>,
                            }
                        },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct OrderItem {
            pub id: i64,
            pub order_id: i64,
            pub product_id: i64,
            pub sku_id: i64,
            pub quantity: i32,
            pub price: f64,
        }
        #[automatically_derived]
        impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for OrderItem
        where
            &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            i32: ::sqlx::decode::Decode<'a, R::Database>,
            i32: ::sqlx::types::Type<R::Database>,
            f64: ::sqlx::decode::Decode<'a, R::Database>,
            f64: ::sqlx::types::Type<R::Database>,
        {
            fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
                let id: i64 = row.try_get("id")?;
                let order_id: i64 = row.try_get("order_id")?;
                let product_id: i64 = row.try_get("product_id")?;
                let sku_id: i64 = row.try_get("sku_id")?;
                let quantity: i32 = row.try_get("quantity")?;
                let price: f64 = row.try_get("price")?;
                ::std::result::Result::Ok(OrderItem {
                    id,
                    order_id,
                    product_id,
                    sku_id,
                    quantity,
                    price,
                })
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for OrderItem {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "OrderItem",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "order_id",
                        &self.order_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "product_id",
                        &self.product_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "sku_id",
                        &self.sku_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "quantity",
                        &self.quantity,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "price",
                        &self.price,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct PayLog {
            pub id: i64,
            pub order_id: i64,
            pub pay_request_info: Option<String>,
            pub pay_response: Option<String>,
            pub callback_infos: Option<String>,
            #[serde(with = "rust_shop_core::entity::db_numeric_date")]
            pub pay_time: NaiveDateTime,
        }
        #[automatically_derived]
        impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for PayLog
        where
            &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
            Option<String>: ::sqlx::types::Type<R::Database>,
            Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
            Option<String>: ::sqlx::types::Type<R::Database>,
            Option<String>: ::sqlx::decode::Decode<'a, R::Database>,
            Option<String>: ::sqlx::types::Type<R::Database>,
            NaiveDateTime: ::sqlx::decode::Decode<'a, R::Database>,
            NaiveDateTime: ::sqlx::types::Type<R::Database>,
        {
            fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
                let id: i64 = row.try_get("id")?;
                let order_id: i64 = row.try_get("order_id")?;
                let pay_request_info: Option<String> = row.try_get("pay_request_info")?;
                let pay_response: Option<String> = row.try_get("pay_response")?;
                let callback_infos: Option<String> = row.try_get("callback_infos")?;
                let pay_time: NaiveDateTime = row.try_get("pay_time")?;
                ::std::result::Result::Ok(PayLog {
                    id,
                    order_id,
                    pay_request_info,
                    pay_response,
                    callback_infos,
                    pay_time,
                })
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for PayLog {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "PayLog",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "order_id",
                        &self.order_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "pay_request_info",
                        &self.pay_request_info,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "pay_response",
                        &self.pay_response,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "callback_infos",
                        &self.callback_infos,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "pay_time",
                        {
                            struct __SerializeWith<'__a> {
                                values: (&'__a NaiveDateTime,),
                                phantom: _serde::__private::PhantomData<PayLog>,
                            }
                            impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                fn serialize<__S>(
                                    &self,
                                    __s: __S,
                                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    rust_shop_core::entity::db_numeric_date::serialize(
                                        self.values.0,
                                        __s,
                                    )
                                }
                            }
                            &__SerializeWith {
                                values: (&self.pay_time,),
                                phantom: _serde::__private::PhantomData::<PayLog>,
                            }
                        },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct ShoppingCart {
            pub id: i64,
            pub product_id: i64,
            pub sku_id: i64,
            pub quantity: i32,
            pub user_id: i64,
            #[serde(with = "rust_shop_core::entity::db_numeric_date")]
            pub add_time: NaiveDateTime,
        }
        #[automatically_derived]
        impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for ShoppingCart
        where
            &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            i32: ::sqlx::decode::Decode<'a, R::Database>,
            i32: ::sqlx::types::Type<R::Database>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            NaiveDateTime: ::sqlx::decode::Decode<'a, R::Database>,
            NaiveDateTime: ::sqlx::types::Type<R::Database>,
        {
            fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
                let id: i64 = row.try_get("id")?;
                let product_id: i64 = row.try_get("product_id")?;
                let sku_id: i64 = row.try_get("sku_id")?;
                let quantity: i32 = row.try_get("quantity")?;
                let user_id: i64 = row.try_get("user_id")?;
                let add_time: NaiveDateTime = row.try_get("add_time")?;
                ::std::result::Result::Ok(ShoppingCart {
                    id,
                    product_id,
                    sku_id,
                    quantity,
                    user_id,
                    add_time,
                })
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ShoppingCart {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "ShoppingCart",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "product_id",
                        &self.product_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "sku_id",
                        &self.sku_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "quantity",
                        &self.quantity,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "user_id",
                        &self.user_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "add_time",
                        {
                            struct __SerializeWith<'__a> {
                                values: (&'__a NaiveDateTime,),
                                phantom: _serde::__private::PhantomData<ShoppingCart>,
                            }
                            impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                fn serialize<__S>(
                                    &self,
                                    __s: __S,
                                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    rust_shop_core::entity::db_numeric_date::serialize(
                                        self.values.0,
                                        __s,
                                    )
                                }
                            }
                            &__SerializeWith {
                                values: (&self.add_time,),
                                phantom: _serde::__private::PhantomData::<ShoppingCart>,
                            }
                        },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct Sku {
            pub id: i64,
            pub title: String,
            pub product_id: i64,
            pub price: f64,
            pub is_default: bool,
        }
        #[automatically_derived]
        impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for Sku
        where
            &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            f64: ::sqlx::decode::Decode<'a, R::Database>,
            f64: ::sqlx::types::Type<R::Database>,
            bool: ::sqlx::decode::Decode<'a, R::Database>,
            bool: ::sqlx::types::Type<R::Database>,
        {
            fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
                let id: i64 = row.try_get("id")?;
                let title: String = row.try_get("title")?;
                let product_id: i64 = row.try_get("product_id")?;
                let price: f64 = row.try_get("price")?;
                let is_default: bool = row.try_get("is_default")?;
                ::std::result::Result::Ok(Sku {
                    id,
                    title,
                    product_id,
                    price,
                    is_default,
                })
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Sku {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Sku",
                        false as usize + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "title",
                        &self.title,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "product_id",
                        &self.product_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "price",
                        &self.price,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "is_default",
                        &self.is_default,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct UserShippingAddress {
            pub id: i64,
            pub user_id: i64,
            pub recipient: String,
            pub phone_number: String,
            pub address: String,
            pub post_code: String,
            pub is_default: bool,
            #[serde(with = "rust_shop_core::entity::db_numeric_date")]
            pub created_time: NaiveDateTime,
        }
        #[automatically_derived]
        impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for UserShippingAddress
        where
            &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            String: ::sqlx::decode::Decode<'a, R::Database>,
            String: ::sqlx::types::Type<R::Database>,
            bool: ::sqlx::decode::Decode<'a, R::Database>,
            bool: ::sqlx::types::Type<R::Database>,
            NaiveDateTime: ::sqlx::decode::Decode<'a, R::Database>,
            NaiveDateTime: ::sqlx::types::Type<R::Database>,
        {
            fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
                let id: i64 = row.try_get("id")?;
                let user_id: i64 = row.try_get("user_id")?;
                let recipient: String = row.try_get("recipient")?;
                let phone_number: String = row.try_get("phone_number")?;
                let address: String = row.try_get("address")?;
                let post_code: String = row.try_get("post_code")?;
                let is_default: bool = row.try_get("is_default")?;
                let created_time: NaiveDateTime = row.try_get("created_time")?;
                ::std::result::Result::Ok(UserShippingAddress {
                    id,
                    user_id,
                    recipient,
                    phone_number,
                    address,
                    post_code,
                    is_default,
                    created_time,
                })
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for UserShippingAddress {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "UserShippingAddress",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "user_id",
                        &self.user_id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "recipient",
                        &self.recipient,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "phone_number",
                        &self.phone_number,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "address",
                        &self.address,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "post_code",
                        &self.post_code,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "is_default",
                        &self.is_default,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "created_time",
                        {
                            struct __SerializeWith<'__a> {
                                values: (&'__a NaiveDateTime,),
                                phantom: _serde::__private::PhantomData<
                                    UserShippingAddress,
                                >,
                            }
                            impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                fn serialize<__S>(
                                    &self,
                                    __s: __S,
                                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    rust_shop_core::entity::db_numeric_date::serialize(
                                        self.values.0,
                                        __s,
                                    )
                                }
                            }
                            &__SerializeWith {
                                values: (&self.created_time,),
                                phantom: _serde::__private::PhantomData::<
                                    UserShippingAddress,
                                >,
                            }
                        },
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub struct Promotion {
            pub id: i64,
        }
        #[automatically_derived]
        impl<'a, R: ::sqlx::Row> ::sqlx::FromRow<'a, R> for Promotion
        where
            &'a ::std::primitive::str: ::sqlx::ColumnIndex<R>,
            i64: ::sqlx::decode::Decode<'a, R::Database>,
            i64: ::sqlx::types::Type<R::Database>,
        {
            fn from_row(row: &'a R) -> ::sqlx::Result<Self> {
                let id: i64 = row.try_get("id")?;
                ::std::result::Result::Ok(Promotion { id })
            }
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for Promotion {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "Promotion",
                        false as usize + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "id",
                        &self.id,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
    }
}
mod extensions {}
mod filter {
    mod jwt_auth_filter {}
}
mod request {
    pub(crate) mod login_request {
        pub struct LoginRequest {
            username: String,
            password: String,
        }
    }
    mod page_query_request {
        pub trait PageQueryRequest {
            fn get_page_index(&self) -> Option<u32>;
            fn set_page_index(&mut self, page_index: Option<u32>);
            fn get_page_size(&self) -> Option<u32>;
            fn set_page_size(&mut self, page_size: Option<u32>);
        }
    }
}
pub mod service {
    pub mod product_category_service {
        use std::borrow::BorrowMut;
        use std::fmt::Error;
        use chrono::Local;
        use sqlx::{MySql, MySqlPool};
        use uuid::Uuid;
        use rust_shop_core::db::{
            mysql_connection_pool, SqlCommandExecutor, TransactionManager,
        };
        use rust_shop_core::id_generator::ID_GENERATOR;
        use rust_shop_core::jwt_service::DefaultJwtService;
        use crate::entity::entity::ProductCategory;
        pub struct ProductCategoryService<'a, 'b> {
            sql_command_executor: &'b mut SqlCommandExecutor<'a, 'b>,
        }
        impl<'a, 'b> ProductCategoryService<'a, 'b> {
            pub fn new(
                sql_command_executor: &'b mut SqlCommandExecutor<'a, 'b>,
            ) -> Self {
                ProductCategoryService {
                    sql_command_executor,
                }
            }
            pub async fn list_all_categories(
                &mut self,
            ) -> anyhow::Result<Vec<ProductCategory>> {
                let categories = self
                    .sql_command_executor
                    .find_all("SELECT * FROM product_category")
                    .await?;
                {
                    ::std::io::_print(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "\u{67e5}\u{8be2}\u{5230}\u{7684}\u{6570}\u{636e}\u{6709}",
                                "\u{6761}\n",
                            ],
                            &[::core::fmt::ArgumentV1::new_display(&categories.len())],
                        ),
                    );
                };
                let jwt_service = DefaultJwtService::new(self.sql_command_executor);
                Ok(categories)
            }
            pub async fn test_tran(&mut self) -> anyhow::Result<()> {
                Ok(())
            }
        }
    }
}
mod state {}
pub mod utils {}
mod vo {}
pub struct AuthFilter;
impl Filter for AuthFilter {
    #[allow(
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn handle<'a, 'async_trait>(
        &'a self,
        ctx: RequestCtx,
        next: Next<'a>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = anyhow::Result<hyper::Response<hyper::Body>>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'a: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret)
                = ::core::option::Option::None::<
                    anyhow::Result<hyper::Response<hyper::Body>>,
                > {
                return __ret;
            }
            let __self = self;
            let mut ctx = ctx;
            let next = next;
            let __ret: anyhow::Result<hyper::Response<hyper::Body>> = {
                let endpoint_result: EndpointResult<String> = EndpointResult::server_error(
                    "鏃犳潈闄?,
                );
                Ok(ResponseBuilder::with_endpoint_result(endpoint_result))
            };
            #[allow(unreachable_code)] __ret
        })
    }
    fn url_patterns(&self) -> String {
        ::core::panicking::panic("not yet implemented")
    }
}
fn load_user_service_fn<'r, 'a, 'b>(
    sql_command_executor: &'r mut SqlCommandExecutor<'a, 'b>,
) -> Box<dyn LoadUserService + Send + Sync + 'r> {
    WeChatUserService::new(sql_command_executor)
}
fn main() -> anyhow::Result<()> {
    register_routes();
    let body = async {
        let mut file = File::open("D:\\椤圭洰\\rust-shop\\src\\api\\auth_controller.rs")
            .expect("Unable to open file");
        let mut src = String::new();
        file.read_to_string(&mut src).expect("Unable to read file");
        let syntax = syn::parse_file(&src).expect("Unable to parse file");
        {
            ::std::io::_print(
                ::core::fmt::Arguments::new_v1_formatted(
                    &["", "\n"],
                    &[::core::fmt::ArgumentV1::new_debug(&syntax)],
                    &[
                        ::core::fmt::rt::v1::Argument {
                            position: 0usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                flags: 4u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Implied,
                            },
                        },
                    ],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ),
            );
        };
        log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1(&["booting up"], &[]),
                    lvl,
                    &("rust_shop", "rust_shop", "src\\main.rs", 91u32),
                    ::log::__private_api::Option::None,
                );
            }
        };
        let addr: SocketAddr = {
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &["127.0.0.1:"],
                    &[::core::fmt::ArgumentV1::new_display(&&APP_CONFIG.server.port)],
                ),
            );
            res
        }
            .parse()
            .unwrap();
        let mut srv = Server::new();
        srv.filter(AccessLogFilter);
        let conn_pool = mysql_connection_pool().await?;
        srv.extension(State::new(conn_pool.clone()));
        let mut security_config = SecurityConfig::new();
        security_config.enable_security(false);
        security_config
            .authentication_token_resolver(
                AuthenticationTokenResolverFn::from(
                    Box::new(|| -> Box<dyn AuthenticationTokenResolver + Send + Sync> {
                        Box::new(WeChatMiniAppAuthenticationTokenResolver {
                        })
                    }),
                ),
            );
        security_config.password_encoder(Box::new(NopPasswordEncoder {}));
        security_config
            .load_user_service(
                LoadUserServiceFn::from(
                    Box::new(|
                        req: &mut RequestCtx,
                    | -> Box<
                        dyn for<'r, 'c, 'd> Fn(
                            &'r mut SqlCommandExecutor<'c, 'd>,
                        ) -> Box<(dyn LoadUserService + Send + Sync + 'r)> + Send + Sync,
                    > { Box::new(load_user_service_fn) }),
                ),
            );
        srv.security_config(security_config);
        srv.post("/", &IndexController::index);
        srv.run(addr).await.unwrap();
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1(&["server shutdown!"], &[]),
                    lvl,
                    &("rust_shop", "rust_shop", "src\\main.rs", 138u32),
                    ::log::__private_api::Option::None,
                );
            }
        };
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
pub fn register_routes() {
    register_route(
        "POST".to_string(),
        "/user/:id/:age".to_string(),
        auth_controller::AuthController::test_handler_proxy,
    );
}
