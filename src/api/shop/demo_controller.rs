pub mod demo_controller {

    use std::string::ToString;

    use anyhow::anyhow;
    use chrono::Local;

    use sqlx::mysql::MySqlArguments;
    use sqlx::{Arguments, MySql, Pool, Row};
    use uuid::Uuid;

    use rust_shop_core::db::SqlCommandExecutor;

    use rust_shop_core::extract::form::Form;
    use rust_shop_core::extract::header::Header;
    use rust_shop_core::extract::json::Json;
    use rust_shop_core::extract::path_variable::PathVariable;
    use rust_shop_core::extract::query::Query;
    use rust_shop_core::extract::request_param::RequestParam;

    use rust_shop_core::extract::FromRequest;
    use rust_shop_core::id_generator::ID_GENERATOR;

    use rust_shop_core::state::State;
    use rust_shop_core::RequestCtx;

    use rust_shop_macro::route;

    use rust_shop_core::db::TransactionManager;
    use rust_shop_core::entity::AdminUser;
    use rust_shop_core::mode_and_view::ModelAndView;
    use rust_shop_core::response::into_response::IntoResponse;
    use rust_shop_core::response::Response;
    use rust_shop_core::session::Session;
    use rust_shop_core::APP_EXTENSIONS;
    use rust_shop_core::db::traits::Crud;
    use rust_shop_core::extract::multipart::Multipart;
    use crate::entity::ProductCategory;

    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    pub struct User {
        pub id: u32,
        pub name: String,
        pub is_auth: bool,
    }
    #[route("POST", "/hello")]
    pub async fn hello<'db,'a>(RequestParam(user): RequestParam<String>,sql_exe: &'a mut SqlCommandExecutor<'db, 'a>,)-> anyhow::Result<String>{
        let mut product_service = ProductService::new(sql_exe);
        let page_query = ProductPageQueryRequest{
            page_size: 10,
            page_index: 1,
            keyword: None,
            category_id: None,
            status: None,
            sort: None,
        };
        let page = product_service.page(&page_query).await?;
        println!("{:?}",page);
        let u = user.clone();
        Ok(format!("hello {}",u))
    }
    #[route("POST", "/user/:id/:age")]
    pub async fn test(
        req: &mut RequestCtx,
        Header(token): Header<Option<String>>,
        Header(cookie): Header<String>,
        PathVariable(id): PathVariable<Option<u32>>,
        PathVariable(age): PathVariable<u32>,
        RequestParam(name): RequestParam<Option<String>>,
        RequestParam(address): RequestParam<String>,
        Form(user): Form<User>,
        Query(user1): Query<User>,
        sql_exe_with_tran: &mut SqlCommandExecutor<'_, '_>,
        mut multipart:Multipart
    ) -> anyhow::Result<Json<User>> {
        let username = req
            .authentication
            .get_authentication_token()
            .get_principal()
            .to_string();
        let u = User {
            id: id.unwrap(),
            name: username,
            is_auth: req.authentication.is_authenticated().clone(),
        };
        //let result = sql_command_executor.execute("").await?;
        let id = ID_GENERATOR.lock().unwrap().real_time_generate();
        let wx_open_id = Uuid::new_v4().to_string();
        let mut args = MySqlArguments::default();
        args.add(id);
        args.add(wx_open_id);
        args.add(1);
        args.add(Local::now());
        let result = sql_exe_with_tran
            .execute_with(
                "insert into user(id,wx_open_id,enable,created_time) values(?,?,?,?)",
                args,
            )
            .await?;

        let id = ID_GENERATOR.lock().unwrap().real_time_generate();
        let wx_open_id = Uuid::new_v4().to_string();
        let mut args = MySqlArguments::default();
        args.add(id);
        args.add(wx_open_id);
        args.add(1);
        args.add(Local::now());
        let result = sql_exe_with_tran
            .execute_with(
                "insert into user(id,wx_open_id,enable,created_time) values(?,?,?,?)",
                args,
            )
            .await?;

        let id = ID_GENERATOR.lock().unwrap().real_time_generate();
        let wx_open_id = Uuid::new_v4().to_string();
        let mut args = MySqlArguments::default();
        args.add(id);
        args.add(wx_open_id);
        args.add(1);
        args.add(Local::now());
        let result = sql_exe_with_tran
            .execute_with(
                "insert into user(id,wx_open_id,enable,created_time) values(?,?,?,?)",
                args,
            )
            .await?;
        let admins: Vec<AdminUser> = sql_exe_with_tran
            .find_all("select * from admin_user")
            .await?;
        let mut args = MySqlArguments::default();
        args.add("admin");
        let admins: Option<AdminUser> = sql_exe_with_tran
            .find_option_with("select * from admin_user where username=?", args)
            .await?;

        req.session.insert_or_update("user".to_string(), &u);

        Ok(Json(u))
    }
    use inflector::Inflector;
    use crate::qo::ProductPageQueryRequest;
    use crate::service::product_service::ProductService;

    #[route("GET", "model_and_view")]
    pub async fn model_and_view(ctx: &mut RequestCtx,sql_exe_with_tran: &mut SqlCommandExecutor<'_, '_>,) -> anyhow::Result<ModelAndView> {


        let id = ID_GENERATOR.lock().unwrap().real_time_generate();

        let mut category = ProductCategory{
            id,
            name: "test".to_string(),
            parent_id: None,
            icon: None,
            pic: None,
            sort_index: 0,
            created_time: Default::default(),
            is_deleted: false,
        };
        let result = category.create(sql_exe_with_tran).await?;
        println!("{:?}",result);
        let c = ProductCategory::select_by_id(id,sql_exe_with_tran).await?;
        if c.is_some() {
            println!("ProductCategory = {:?}",c.unwrap());
        }
        category.name = String::from("test1");
        category.icon = Some(String::from("icon"));
        let result = category.update(sql_exe_with_tran).await?;
        let result = ProductCategory::delete_by_id(category.id,sql_exe_with_tran).await?;

        let mut model_and_view = ModelAndView::new("test.html".to_string());
        let user = User {
            id: 0,
            name: "pgg".to_string(),
            is_auth: false,
        };
        model_and_view.insert("user", &user);
        ctx.session.insert_or_update("user".to_string(), &user);
        Ok(model_and_view)
    }
}
