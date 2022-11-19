

pub mod AuthController {
    use std::any::Any;
    use std::borrow::BorrowMut;
    use std::string::ToString;
    use std::thread;
    use rust_shop_core::{EndpointResult, RequestStateResolver, ResponseBuilder};
    use rust_shop_core::extract::json::Json;
    use rust_shop_macro::route;
    use lazy_static::lazy_static;
    use rust_shop_core::RequestCtx;
    use hyper::Response;
    use hyper::Body;
    use rust_shop_core::router::register_route;
    use rust_shop_core::extract::FromRequest;
    use rust_shop_core::extract::IntoResponse;
    use rust_shop_core::extract::path_variable::PathVariable;
    use rust_shop_core::extract::request_param::RequestParam;
    use crate::StatusCode;
    use anyhow::anyhow;
    use std::convert::Infallible;
    use std::ops::Deref;
    use std::sync::Arc;
    use sqlx::{Arguments, MySql, Pool, Row};
    use uuid::Uuid;
    use rust_shop_core::db::DbPoolManager;
    use rust_shop_core::extensions::Extensions;
    use rust_shop_core::extract::extension::Extension;
    use rust_shop_core::extract::form::Form;
    use rust_shop_core::extract::header::Header;
    use rust_shop_core::extract::query::Query;
    use rust_shop_core::extract::request_state::RequestState;
    use rust_shop_core::id_generator::ID_GENERATOR;
    use rust_shop_core::security::UserDetails;
    use rust_shop_core::state::State;
    use crate::entity::entity::ProductCategory;
    use crate::service::product_category_service::ProductCategoryService;
    use chrono::Local;

    #[derive(serde::Serialize,serde::Deserialize,Debug)]
    pub struct User{
        pub id:u32,
        pub name:String
    }

    #[route("POST","/user/:id/:age")]
    pub async fn test(extensions:Arc<Extensions>,
                      request_states:Arc<Extensions>,
                      Header(token):Header<Option<String>>,
                      Header(cookie):Header<String>,
                      PathVariable(id):PathVariable<Option<u32>>,
                      PathVariable(age):PathVariable<u32>,
                      RequestParam(name):RequestParam<Option<String>>,
                      RequestParam(address):RequestParam<String>,
                      Form(user):Form<User>,
                      pool: &mut DbPoolManager<'_, MySql>) ->anyhow::Result<Json<User>>{
        pool.begin().await?;

        let mut p = ProductCategoryService::new(pool);

        p.test_tran().await?;



        let user_id = ID_GENERATOR.lock().unwrap().real_time_generate();
        let wx_open_id = Uuid::new_v4().to_string();


        let rows_affected = sqlx::query!("insert into `user`(id,wx_open_id,created_time,enable) values(?,?,?,?)",user_id,wx_open_id,Local::now(),1)
            .execute(pool.get_pool()).await?
            .rows_affected();

        let user_id = ID_GENERATOR.lock().unwrap().real_time_generate();
        let wx_open_id = Uuid::new_v4().to_string();
        let rows_affected = sqlx::query!("insert into `user`(id,wx_open_id,created_time,enable) values(?,?,?,?)",user_id,wx_open_id,Local::now(),1)
            .execute(pool.get_pool()).await?
            .rows_affected();

        let mut args = sqlx::mysql::MySqlArguments::default();
        args.add(1);
        let result:Vec<ProductCategory> = sqlx::query_as_with("select * from product_category where id = ?",args)
            .fetch_all(pool.get_pool()).await?;

        println!("{:#?}",*result.get(0).unwrap());


        if true {
            return Err(anyhow!("我故意抛出异常的"));
        }

        let user_id = ID_GENERATOR.lock().unwrap().real_time_generate();
        let wx_open_id = Uuid::new_v4().to_string();
        let rows_affected = sqlx::query!("insert into `user`(id,wx_open_id,created_time,enable) values(?,?,?,?)",user_id,wx_open_id,Local::now(),1)
            .execute(pool.get_pool()).await?
            .rows_affected();

        let categories = sqlx::query_as!(ProductCategory,"SELECT * FROM product_category")
            .fetch_all(pool.get_pool()).await?;
        println!("查询到的数据有{}条",categories.len());

        println!("token={:?}",token);
        println!("cookie={:?}",cookie);
        println!("id={:?}",id);
        println!("age={:?}",age);
        println!("name={:?}",name);
        println!("address={:?}",address);
        let u = User{
            id:id.unwrap(),
            name:name.unwrap()
        };
        if token.is_none() {
            println!("token={}","None");
        }else {
            println!("token={}",token.unwrap());
        }
        println!("{:?}",u);

        Ok(Json(u))
    }
   /* pub async fn test_handler_proxy(
        ctx: RequestCtx,
    ) -> anyhow::Result<Response<Body>> {
        let pool:&State<Pool<MySql>> = ctx.extensions.get().unwrap();
        let mut db_pool: DbPoolManager<MySql> = DbPoolManager::new(pool.clone());

        let extensions: Arc<Extensions> = ctx.extensions.clone();
        let request_states: Arc<Extensions> = ctx.request_states.clone();
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
                anyhow!("")
            );
        } else {
            let cookie_tmp_var_2 = cookie_tmp_var_2.unwrap();
            if cookie_tmp_var_2.is_none() {
                return Err(
                    anyhow!("")
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
                    anyhow!("")
                );
            } else {
                id = PathVariable(Some(id_tmp_var.unwrap()));
            }
        }
        let mut age: Option<PathVariable<u32>> = None;
        let age_tmp_var = ctx.router_params.find("age");
        if age_tmp_var.is_none() {
            return Err(
                anyhow!("")
            );
        } else {
            let parse_result = age_tmp_var.unwrap().to_string().parse::<u32>();
            if parse_result.is_err() {
                return Err(
                    anyhow!("")
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
                    anyhow!("")
                );
            } else {
                name = RequestParam(Some(name_tmp_var.unwrap()));
            }
        }
        let mut address: Option<RequestParam<String>> = None;
        let address_tmp_var = ctx.query_params.get("address");
        if address_tmp_var.is_none() {
            return Err(
                anyhow!("")
            );
        } else {
            let parse_result = address_tmp_var
                .unwrap()
                .to_string()
                .parse::<String>();
            if parse_result.is_err() {
                return Err(
                    anyhow!("")
                );
            } else {
                address = Some(RequestParam(parse_result.unwrap()));
            }
        }
        let address = address.unwrap();
        let user = Form::from_request(ctx).await?;
        let handler_result = test(
            extensions,
            request_states,
            token,
            cookie,
            id,
            age,
            name,
            address,
            user,
            &db_pool
        )
            .await;



            return if handler_result.is_err() {
                if db_pool.use_tran() {
                    db_pool.rollback().await?;
                }
                Err(handler_result.err().unwrap())
            } else {
                if db_pool.use_tran() {
                    db_pool.commit().await?;
                }
                Ok(handler_result.unwrap().into_response())
            }

    }*/
}



    //#[route("POST","/user")]
 /*   pub async fn test_handler11(req:RequestCtx,extensions:&Arc<Extensions>,)->anyhow::Result<Json<User>>{
        let mut db_pool:&DbPoolManager<MySql> = RequestStateResolver::get(&req);
        if db_pool.use_tran() {
            db_pool.begin();
        }
        Ok(Json(User{
            id: 0,
            name: "pgg".to_string()
        }))
    }*/

