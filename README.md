# rust-shop
rust-shop是使用rust语言开发的微信商城（商城功能开发中），内含web开发框架，使用简单，可扩展。
# web开发框架特性
#### 1、注解式自动注册路由。自动提取参数，自动提取的参数包含：Header、PathVariable、RequestParam、Form、Query、Json等
```rust
    #[route("POST", "/hello")]
    pub async fn hello(RequestParam(user): RequestParam<String>)-> anyhow::Result<String>{
        let u = user.clone();
        Ok(format!("hello {}",u))
    }
```
上面的代码匹配的请求为：
POST /hello?user=pgg
#### 2、自动事务。一个请求视为一个事务，框架实现了请求的整体提交或者整体回滚，当发生异常时，自动回滚事务，如果请求没有异常发生，则自动提交事务。也可以实现事务嵌套。
```rust 
    #[route("POST", "/hello")]
    pub async fn hello(sql_exe_with_tran: &mut SqlCommandExecutor<'_, '_>)-> anyhow::Result<String>{
        Ok(String::from("hello"))
    }
``` 
如果不想使用事务，只需将sql_exe_with_tran参数名改为sql_exe即可
#### 3、内置登录、权限模块。只需改变配置即可替换原框架的默认实现。
```rust
   let mut security_config = WebSecurityConfigurer::new();
    security_config.enable_security(false);
    security_config.authentication_token_resolver(AuthenticationTokenResolverFn::from(Box::new(
        || -> Box<dyn AuthenticationTokenResolver + Send + Sync> {
            Box::new(UsernamePasswordAuthenticationTokenResolver {})
        },
    )));
    security_config.password_encoder(Box::new(Sha512PasswordEncoder));
    security_config.load_user_service(LoadUserServiceFn::from(Box::new(
        |_req: &mut RequestCtx| -> Box<
            dyn for<'r, 'c, 'd> Fn(
                    &'r mut SqlCommandExecutor<'c, 'd>,
                ) -> Box<(dyn LoadUserService + Send + Sync + 'r)>
                + Send
                + Sync,
        > { Box::new(load_user_service_fn) },
    )));
    srv.security_config(security_config);
```
#### 4、http handler支持丰富的返回值类型，如：JSON,String,Bytes等
#### 5、支持session及cookie
#### 6、支持模板
```rust
    #[route("GET", "model_and_view")]
    pub async fn model_and_view(ctx: &mut RequestCtx) -> anyhow::Result<ModelAndView> {
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
```
#### 7、支持http请求处理中间件（类似java spring mvc中的filter）
#### 8、进一步封装了sqlx，数据库访问操作更简便
# 使用方法
```rust
#[tokio::main]
#[rust_shop_macro::scan_route("/src")]
async fn main() -> anyhow::Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("booting up");

    let addr: SocketAddr = format!("127.0.0.1:{}", &APP_CONFIG.server.port)
        .parse()
        .unwrap();

    let mut srv = Server::new();

    srv.filter(AuthenticationFilter);
    srv.filter(AccessLogFilter);
    srv.filter(AuthenticationProcessingFilter);
    srv.filter(SecurityInterceptor);

    let conn_pool = mysql_connection_pool().await?;
    srv.extension(State::new(conn_pool.clone()));

    let mut security_config = WebSecurityConfigurer::new();
    security_config.enable_security(false);
    security_config.authentication_token_resolver(AuthenticationTokenResolverFn::from(Box::new(
        || -> Box<dyn AuthenticationTokenResolver + Send + Sync> {
            Box::new(UsernamePasswordAuthenticationTokenResolver {})
        },
    )));
    security_config.password_encoder(Box::new(Sha512PasswordEncoder));
    security_config.load_user_service(LoadUserServiceFn::from(Box::new(
        |_req: &mut RequestCtx| -> Box<
            dyn for<'r, 'c, 'd> Fn(
                    &'r mut SqlCommandExecutor<'c, 'd>,
                ) -> Box<(dyn LoadUserService + Send + Sync + 'r)>
                + Send
                + Sync,
        > { Box::new(load_user_service_fn) },
    )));
    srv.security_config(security_config);
    srv.run(addr).await.unwrap();

    info!("server shutdown!");
    Ok(())
}
```
