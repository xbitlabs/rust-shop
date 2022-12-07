### rust-shop是使用rust语言开发的微信商城，内含一个rust语言开发的web框架。
## web开发框架特色：
#### 1、注解式自动注册路由。自动提取参数，自动提取的参数包含：Header、PathVariable、RequestParam、Form、Query
```
    #[route("POST", "/hello")]
    pub async fn hello(RequestParam(user): RequestParam<String>)-> anyhow::Result<String>{
        let u = user.clone();
        Ok(format!("hello {}",u))
    }
```
匹配的请求为：
POST /hello?user=pgg
#### 2、自动事务。一个请求视为一个事务，框架实现了请求的整体提交或者整体回滚，当发生异常时，自动回滚事务，如果请求没有异常发生，则自动提交事务。也可以实现事务嵌套。
```
    #[route("POST", "/hello")]
    pub async fn hello(sql_exe_with_tran: &mut SqlCommandExecutor<'_, '_>)-> anyhow::Result<String>{
        Ok(String::from("hello"))
    }
```
### 3、内置认证、权限模块。只需改变配置即可替换原框架的默认实现。
```
```
