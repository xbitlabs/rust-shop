extern crate core;

use proc_macro::{Span, TokenStream};
use std::any::{Any, TypeId};
use std::iter::FromIterator;
use chrono::NaiveDateTime;
use syn::{Data, DeriveInput, Fields, FnArg, GenericArgument, Ident, Item, ItemFn, parse_macro_input, Pat, Path, PathArguments, PatTuple, PatType, Type, TypePath};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use thiserror::Error;
use std::alloc::System;
use std::{env, fs};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use proc_macro2::TokenTree;
use quote::{quote, TokenStreamExt, ToTokens};
use quote::__private::ext::RepToTokensExt;
use syn::spanned::Spanned;
use syn::parse_quote;
use uuid::Uuid;


fn is_option_f32_type(type_name:&String)->bool{
    return type_name == "Option<f32>";
}
fn is_option_f64_type(type_name:&String)->bool{
    return type_name == "Option<f64>";
}
fn is_option_i8_type(type_name:&String)->bool{
    return type_name == "Option<i8>";
}
fn is_option_u8_type(type_name:&String)->bool{
    return type_name == "Option<u8>";
}
fn is_option_i16_type(type_name:&String)->bool{
    return type_name == "Option<i16>";
}
fn is_option_u16_type(type_name:&String)->bool{
    return type_name == "Option<u16>";
}
fn is_option_i32_type(type_name:&String)->bool{
    return type_name == "Option<i32>";
}
fn is_option_u32_type(type_name:&String)->bool{
    return type_name == "Option<u32>";
}
fn is_option_i64_type(type_name:&String)->bool{
    return type_name == "Option<i64>";
}
fn is_option_u64_type(type_name:&String)->bool{
    return type_name == "Option<u64>";
}
fn is_option_i128_type(type_name:&String)->bool{
    return type_name == "Option<i128>";
}
fn is_option_u128_type(type_name:&String)->bool{
    return type_name == "Option<u128>";
}
fn is_option_isize_type(type_name:&String)->bool{
    return type_name == "Option<isize>";
}
fn is_option_usize_type(type_name:&String)->bool{
    return type_name == "Option<usize>";
}
fn is_option_bool_type(type_name:&String)->bool{
    return type_name == "Option<bool>";
}
fn is_option_datetime_type(type_name:&String)->bool{
    return type_name == "Option<NaiveDateTime>";
}
fn is_option_date_type(type_name:&String)->bool{
    return type_name == "Option<NaiveDate>";
}
fn is_option_string_type(type_name:&String)->bool{
    return type_name == "Option<String>";
}


fn is_datetime_type(type_name:&String)->bool{
    return type_name == "NaiveDateTime";
}
fn is_string_type(type_name:&String)->bool{
    return type_name == "String";
}
fn is_date_type(type_name:&String)->bool{
    return type_name == "NaiveDate";
}
fn map_fields<F>(fields: &Fields, mapper: F) -> TokenStream2
    where
        F: FnMut((&Ident, &Type)) -> TokenStream2,
{
    TokenStream2::from_iter(
        fields
            .iter()
            .map(|field| (field.ident.as_ref().unwrap(), &field.ty))
            .map(mapper),
    )
}
#[proc_macro_derive(FormParser)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let ident_builder = Ident::new(&format!("{}FormParser", ident), ident.span());
    if let Data::Struct(r#struct) = input.data {
        let fields = r#struct.fields;
        if matches!(&fields, Fields::Named(_)) {
            let builder_fields = map_fields(&fields, |(ident, ty)| quote!(#ident: #ty, ));
            /*let builder_set_fields = map_fields(&fields, |(ident, ty)| {
                quote!(pub fn #ident(mut self, value: #ty) -> Self {
                    self.#ident = value;
                    self
                })
            });*/
            let build_lets = map_fields(&fields, |(ident, ty)| {
                //is_datetime_type
                //is_string_type
                let type_name = ty.to_token_stream().to_string().replace(" ","");
                if type_name.starts_with("Option<") {
                    if is_option_f32_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = param.unwrap().parse::<f32>();
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }else if is_option_f64_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = param.unwrap().parse::<f64>();
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_i8_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = param.unwrap().parse::<i8>();
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_u8_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = param.unwrap().parse::<u8>();
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_i16_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = param.unwrap().parse::<i16>();
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_u16_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = param.unwrap().parse::<u16>();
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_i32_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = param.unwrap().parse::<i32>();
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_u32_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = param.unwrap().parse::<u32>();
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_i64_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = param.unwrap().parse::<i64>();
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_u64_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = param.unwrap().parse::<u64>();
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_i128_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = param.unwrap().parse::<i128>();
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_u128_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = param.unwrap().parse::<u128>();
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_isize_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = param.unwrap().parse::<isize>();
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_usize_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = param.unwrap().parse::<usize>();
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_bool_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = param.unwrap().parse::<bool>();
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_datetime_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = NaiveDateTime::parse_from_str(param.unwrap(), "%Y-%m-%d %H:%M:%S");
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_date_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                let parse_result = NaiveDate::parse_from_str(param.unwrap(), "%Y-%m-%d");
                                if parse_result.is_ok(){
                                    #ident = Some(parse_result.unwrap());
                                }
                            }
                        )
                    }
                    else if is_option_string_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                #ident = Some(param.unwrap().to_string());
                            }
                        )
                    }
                    else {
                        quote!()
                    }

                }else {
                    if is_datetime_type(&type_name) {
                        quote!(
                            //let type_name = #type_name;
                            let param = form_params.get(stringify!(#ident));
                            if param.is_none() {
                                return Err(anyhow!(format!(
                                            "field \"{}\" required, but not set yet.",
                                            stringify!(#ident),
                                        )))
                            }
                            let parse_result = NaiveDateTime::parse_from_str(param.unwrap(), "%Y-%m-%d %H:%M:%S");
                            if parse_result.is_err(){
                                return Err(anyhow!(format!(
                                            "field \"{}\" value is invalid.",
                                            stringify!(#ident),
                                        )))
                            }
                            let #ident = parse_result.unwrap();
                        )
                    }else if is_date_type(&type_name){
                        quote!(
                            //let type_name = #type_name;
                            let param = form_params.get(stringify!(#ident));
                            if param.is_none() {
                                return Err(anyhow!(format!(
                                            "field \"{}\" required, but not set yet.",
                                            stringify!(#ident),
                                        )))
                            }
                            let parse_result = NaiveDate::parse_from_str(param.unwrap(), "%Y-%m-%d");
                            if parse_result.is_err(){
                                return Err(anyhow!(format!(
                                            "field \"{}\" value is invalid.",
                                            stringify!(#ident),
                                        )))
                            }
                            let #ident = parse_result.unwrap();
                        )
                    }
                    //不是字符串
                    else if !is_string_type(&type_name){
                        quote!(
                            //let type_name = #type_name;
                            let param = form_params.get(stringify!(#ident));
                            if param.is_none() {
                                return Err(anyhow!(format!(
                                            "field \"{}\" required, but not set yet.",
                                            stringify!(#ident),
                                        )))
                            }
                            let parse_result = param.unwrap().parse::<#ty>();
                            if parse_result.is_err(){
                                return Err(anyhow!(format!(
                                            "field \"{}\" value is invalid.",
                                            stringify!(#ident),
                                        )))
                            }
                            let #ident = parse_result.unwrap();
                        )
                    }else {
                        quote!(
                            //let type_name = #type_name;
                            let param = form_params.get(stringify!(#ident));
                            if param.is_none() {
                                return Err(anyhow!(format!(
                                            "field \"{}\" required, but not set yet.",
                                            stringify!(#ident),
                                        )))
                            }
                            let #ident = param.unwrap().to_string();
                        )
                    }
                }

            });
            let build_values = map_fields(&fields, |(ident, _)| quote!(#ident,));
            let result = quote!(
                impl #ident {
                    pub fn build_form_parser() -> #ident_builder {
                        #ident_builder::default()
                    }
                }

                #[derive(Default)]
                pub struct #ident_builder {
                    #builder_fields
                }

                impl #ident_builder {
                    //#builder_set_fields

                    pub async fn parse(self,req:Request<Body>) -> anyhow::Result<#ident> {
                        let form_params = parse_form_params(req).await;
                        #build_lets
                        Ok(#ident { #build_values })
                    }
                }
            )
                .into();
            // eprintln!("{}", result);
            return result;
        }
    }
    let q = quote!().into();
    eprintln!("{}", q);
    q
}



///
///路由
///
struct Args {
    vars: Vec<syn::Expr>
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let vars = Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated(input)?;

        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

impl Args {
    pub fn get_method(&self) -> syn::Result<syn::Expr> {
        match self.vars.get(0) {
            Some(var) => Ok(var.clone()),
            None => return Err(syn::Error::new(
                Span::call_site().into(),
                "No HTTP Method was provided"
            ))
        }
    }

    pub fn get_route(&self) -> syn::Result<syn::Expr> {
        match self.vars.get(1) {
            Some(var) => Ok(var.clone()),
            None => return Err(syn::Error::new(
                Span::call_site().into(),
                "No Route was provided"
            ))
        }
    }
}

///handler的代理函数后缀
const HANDLER_PROXY_FN_SUFFIX:&str = "_handler_proxy";
///handler的参数
struct HandlerParam{
    ///参数的类型，如：RequestParam,Json,Query等
    pub param_type:String,
    ///参数名，如：RequestParam(id):RequestParam<u32>，这里的id就是参数名
    pub param_name:String,
    ///参数是否是Option，如：RequestParam(id):RequestParam<Option<u32>>，这样的参数就是Option，也有可能不是Option，可以是一些原始类型，如u32,i64,String等
    pub param_option:String,
    ///参数Option的泛型类型，如：RequestParam(id):RequestParam<Option<u32>>，这样的参数的Option的泛型类型就是u32
    pub param_option_type:String
}

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    //宏传入的参数，从里面提取请求方法及路径
    let args = parse_macro_input!(args as Args);
    //方法体
    let func = parse_macro_input!(input as ItemFn);

    let idents = func.sig.inputs.iter().filter_map(|param|{
        if let syn::FnArg::Typed(pat_type) = param {
            if let syn::Pat::Ident(pat_ident) = *pat_type.pat.clone() {
                return Some(pat_ident.ident)
            }
        }

        None
    });
    let mut punctuated: Punctuated<syn::Ident, Comma> = Punctuated::new();
    idents.for_each(|ident| punctuated.push(ident));

    //方法的访问修饰符
    let vis = func.vis.clone();
    //方法签名
    let ident = func.sig.ident.clone();
    //请求方法，如：get,post
    let method = args.get_method().unwrap();
    //请求路径
    let route = args.get_route().unwrap();

    let fn_name = ident.to_string();
    let handler_proxy_name = fn_name.clone() + HANDLER_PROXY_FN_SUFFIX;

    let mut params = vec![];

    for arg in func.sig.inputs.iter() {
        //RequestParam,user,String
        //参数的类型，如：RequestParam,Json,Query等
        let mut handler_param_type = String::from("");
        //参数名，如：RequestParam(id):RequestParam<u32>，这里的id就是参数名
        let mut handler_param_name = String::from("");
        //参数是否是Option，如：RequestParam(id):RequestParam<Option<u32>>，这样的参数就是Option，也有可能不是Option，可以是一些原始类型，如u32,i64,String等
        let mut handler_param_option = String::from("");
        //参数Option的泛型类型，如：RequestParam(id):RequestParam<Option<u32>>，这样的参数的Option的泛型类型就是u32
        let mut handler_param_option_type = String::from("");
        if let FnArg::Typed(pat_type) = arg {
            match &*pat_type.pat {
                Pat::Ident(_) => {
                }
                Pat::TupleStruct(tuple_struct) => {
                    let pat = tuple_struct.pat.elems.first().unwrap();
                    match pat {
                        Pat::Ident(pat_ident) => {
                            handler_param_name = pat_ident.ident.to_string();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            let ty = pat_type.ty.clone();

            match *ty {
                Type::Path(path)=>{
                    let type_name = path.path.segments[0].ident.to_string();
                    let arguments = path.path.segments.first().unwrap();
                    let path_args = &arguments.arguments;
                    match path_args{
                        PathArguments::AngleBracketed(angle_bracketed) => {
                            let arg = angle_bracketed.args.first().unwrap();
                            match arg{
                                GenericArgument::Type(arg_type) => {
                                    match arg_type {
                                        Type::Path(arg_path) => {
                                            handler_param_option = arg_path.path.segments.first().unwrap().ident.to_string();
                                            let path_args = &arg_path.path.segments.first().unwrap().arguments;
                                            match path_args {
                                                PathArguments::AngleBracketed(angle_bracketed) => {
                                                    let arg = angle_bracketed.args.first().unwrap();
                                                    match arg {
                                                        GenericArgument::Type(arg_type) => {
                                                            match arg_type {
                                                                Type::Path(arg_path) => {
                                                                    handler_param_option_type = arg_path.path.segments.first().unwrap().ident.to_string();
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                        _ => {}
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                    if type_name == "Json" {
                        handler_param_type = "Json".to_string();
                    }
                    else if type_name == "Form" {
                        handler_param_type = "Form".to_string();
                    }
                    else if type_name == "Query" {
                        handler_param_type = "Query".to_string();
                    }else if type_name == "Header" {
                        handler_param_type = "Header".to_string();
                    } else if type_name == "PathVariable" {
                        handler_param_type = "PathVariable".to_string();
                    } else if type_name == "RequestParam" {
                        handler_param_type = "RequestParam".to_string();
                    }else if type_name == "RequestCtx" {
                        handler_param_type = "RequestCtx".to_string();
                    } else {
                    }
                }
                _ =>{}
            }
            params.push(HandlerParam{
                param_type: handler_param_type,
                param_name: handler_param_name,
                param_option: handler_param_option,
                param_option_type: handler_param_option_type
            });
        }
    }
    //panic!("{:?}",params);
    if params.is_empty() {
        panic!("handler参数个数为0，请至少传入一个RequestCtx参数");
    }

    let mut handler_proxy_fn_body = String::from("");
    let mut original_fn_inputs = vec![];

    for param in params {
        if param.param_type == "RequestCtx" {
            original_fn_inputs.push(String::from("ctx"));
        }else {
            if param.param_type == "Json" {
                handler_proxy_fn_body = handler_proxy_fn_body + "  let " + &*param.param_name + " = Json::from_request(ctx).await?;\r\n"
            }
            if param.param_type == "Form" {
                handler_proxy_fn_body = handler_proxy_fn_body + "  let " + &*param.param_name + " = Form::from_request(ctx).await?;\r\n"
            }
            if param.param_type == "Query" {
                handler_proxy_fn_body = handler_proxy_fn_body + "  let " + &*param.param_name + " = Query::from_request(ctx).await?;\r\n"
            }
            if param.param_type == "Header" {
                if param.param_option.eq("Option") {
                    let header_tmp_var = param.param_name.clone() + "_tmp_var";
                    handler_proxy_fn_body = handler_proxy_fn_body +
                        &*format!(
                        "let mut {0}:Header<Option<String>> = Header(None);\r\n
                        let {1} = ctx.headers.get(\"{0}\");\r\n
                        if {1}.is_some() {{\r\n
                            let {1} = {1}.unwrap();\r\n
                            if {1}.is_some() {{\r\n
                                {0} = Header(Some({1}.as_ref().unwrap().to_string()));\r\n
                            }}\r\n
                        }}\r\n",
                        param.param_name.clone(),header_tmp_var);
                }else {
                    let msg = format!("header '{}' is None",param.param_name);
                    let header_tmp_var_1 = param.param_name.clone() + "_tmp_var_1";
                    let header_tmp_var_2 = param.param_name.clone() + "_tmp_var_2";
                    handler_proxy_fn_body = handler_proxy_fn_body + &*format!(
                        "let mut {0}:Option<Header<String>> = None;  \r\n
                        let {1} = ctx.headers.get(\"{3}\");          \r\n
                        if {1}.is_none() {{                           \r\n
                            return Err(anyhow!(\"{2}\"));            \r\n
                        }}else{{                                       \r\n
                            let {1} = {1}.unwrap();                  \r\n
                            if {1}.is_none() {{                       \r\n
                                return Err(anyhow!(\"{2}\"));        \r\n
                            }}else {{                                  \r\n
                                {0} = Some(Header({1}.as_ref().unwrap().to_string()));    \r\n
                            }}                                        \r\n
                        }}                                            \r\n
                        let {3}:Header<String> = {0}.unwrap();       \r\n", header_tmp_var_1,header_tmp_var_2, msg,param.param_name);
                }

            }
            if param.param_type == "PathVariable" {
                //panic!("{}",param.2);
                if param.param_option.eq("Option") {
                    let header_tmp_var = param.param_name.clone() + "_tmp_var";
                    let msg = format!("PathVariable '{}' is invalid", param.param_name.clone());
                    handler_proxy_fn_body = handler_proxy_fn_body + &*format!(
                         "let mut {0}:PathVariable<Option<{2}>> = PathVariable(None);
                        let {1} = ctx.router_params.find(\"{0}\");
                        if {1}.is_some() {{
                            let {1} = {1}.unwrap().to_string();
                            let {1} = {1}.parse::<{2}>();
                            if {1}.is_err(){{
                                return Err(anyhow!(\"{3}\"));
                            }}else{{
                                {0} = PathVariable(Some({1}.unwrap()));
                            }}
                        }}",
                        param.param_name, header_tmp_var,param.param_option_type,msg);
                }else {
                    let msg_none = format!("router param '{}' is None",param.param_name);
                    let msg_invalid = format!("router param '{}' is invalid",param.param_name);
                    let header_tmp_var = param.param_name.clone() + "_tmp_var";
                    handler_proxy_fn_body = handler_proxy_fn_body + &*format!(
                        "let mut {0}:Option<PathVariable<{4}>> = None;\r\n
                         let {1} = ctx.router_params.find(\"{0}\");\r\n
                         if {1}.is_none() {{\r\n
                            return Err(anyhow!(\"{2}\"));\r\n
                         }}else {{\r\n
                             let parse_result = {1}.unwrap().to_string().parse::<{4}>();\r\n
                             if parse_result.is_err() {{\r\n
                                return Err(anyhow!(\"{3}\"));\r\n
                             }}else {{\r\n
                                {0} = Some(PathVariable(parse_result.unwrap()));\r\n
                             }}\r\n
                        }}\r\n
                        let {0} = {0}.unwrap();\r\n"
                        , param.param_name, header_tmp_var,msg_none,msg_invalid,param.param_option);
                }
            }
            if param.param_type == "RequestParam" {
                if param.param_option.eq("Option") {
                    let header_tmp_var = param.param_name.clone() + "_tmp_var";
                    let msg = format!("RequestParam '{}' is invalid", param.param_name.clone());
                    handler_proxy_fn_body = handler_proxy_fn_body + &*format!(
                        "let mut {0}:RequestParam<Option<{2}>> = RequestParam(None);
                        let {1} = ctx.query_params.get(\"{0}\");
                        if {1}.is_some() {{
                            let {1} = {1}.unwrap().to_string();
                            let {1} = {1}.parse::<{2}>();
                            if {1}.is_err(){{
                                return Err(anyhow!(\"{3}\"));
                            }}else{{
                                {0} = RequestParam(Some({1}.unwrap()));
                            }}
                        }}",
                        param.param_name, header_tmp_var,param.param_option_type,msg);
                }else {

                    let msg_none = format!("router param '{}' is None",param.param_name);
                    let msg_invalid = format!("router param '{}' is invalid",param.param_name);

                    let header_tmp_var = param.param_name.clone() + "_tmp_var";
                    handler_proxy_fn_body = handler_proxy_fn_body + &*format!(
                        "let mut {0}:Option<RequestParam<{4}>> = None;\r\n
                        let {1} = ctx.query_params.get(\"{0}\");\r\n
                        if {1}.is_none() {{\r\n
                            return Err(anyhow!(\"{2}\"));\r\n
                        }}else {{\r\n
                            let parse_result = {1}.unwrap().to_string().parse::<{4}>();\r\n
                            if parse_result.is_err() {{\r\n
                                return Err(anyhow!(\"{3}\"));\r\n
                            }}else {{\r\n
                                {0} = Some(RequestParam(parse_result.unwrap()));\r\n
                            }}\r\n
                        }}\r\n
                        let {0} = {0}.unwrap();\r\n"
                        , param.param_name, header_tmp_var,msg_none,msg_invalid,param.param_option);
                }
            }
            original_fn_inputs.push(String::from(param.param_name.clone()));
        }
    }
    let mut inputs = String::from("");
    let mut i = 0;
    for original_fn_input in original_fn_inputs {
        if i == 0 {
            inputs = inputs + &*original_fn_input;
        }else {
            inputs = inputs + "," + &*original_fn_input;
        }
        i = i + 1;
    }
    let original_fn_name = ident.to_string();
    let handler_proxy_fn = String::from("pub async fn ") + &*handler_proxy_name + "(ctx:RequestCtx)->anyhow::Result<Response<Body>>{"
     + &*handler_proxy_fn_body +
        "let handler_result = " + &*original_fn_name + "(" + &*inputs + ").await?;\r\n" +
         "Ok(handler_result.into_response())\r\n" +
        "}\r\n";
    //panic!("{:#?}",handler_fn);
    let handler_token_stream = TokenStream::from_str(handler_proxy_fn.as_str()).unwrap();

    let expanded = quote! {
        #func
    };
    TokenStream::from_iter(vec![expanded.into(),handler_token_stream])
}


use walkdir::WalkDir;

///
/// 扫描路由并自动注册路由
///
#[proc_macro_attribute]
pub fn scan_route(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut args = args.to_string();
    if args.is_empty() {
        panic!("必须传入项目的controller源码相对路径，如:/src/controller");
    }else {
        let current_dir = env::current_dir();
        if current_dir.is_err() {
            panic!("无法获取当前目录");
        } else {
            args = args.replace("\"","");
            let current_dir = current_dir.unwrap().to_str().unwrap().to_string().replace("\\","/");
            let mut path = String::from("");
            path = current_dir + &*args;
            let source_path = PathBuf::from(&path);
            if !source_path.exists() {
                panic!("未找到路径：{}", path)
            }
            let mut register_route_fn = String::from("");
            for entry in WalkDir::new(path) {
                let entry = entry.unwrap();
                let file = entry.path().to_str().unwrap().to_string();
                if entry.path().is_file() && file.ends_with("_controller.rs"){
                    let mut file = File::open(file).expect("Unable to open file");

                    let mut src = String::new();
                    file.read_to_string(&mut src).expect("Unable to read file");

                    let syntax = syn::parse_file(&src).expect("Unable to parse file");
                    //println!("{:#?}", syntax);
                    for item in syntax.items {
                        match item {
                            Item::Mod(item_mod)=>{
                                let mod_ident = item_mod.ident;
                                if item_mod.content.is_some() {
                                    //println!("{}",item_mod.content.unwrap().1.len());
                                    let mod_content_items = item_mod.content.unwrap().1;
                                    for mod_content_item in mod_content_items {
                                        match mod_content_item {
                                            Item::Fn(func)=>{
                                                //获取路由信息
                                                let attrs = func.attrs;
                                                let fn_ident = func.sig.ident;
                                                let mut route_method = String::from("");
                                                let mut route_url = String::from("");
                                                let mut has_request_ctx_params = false;
                                                let mut has_form_or_query_or_json = false;
                                                //函数没有route注解就跳过
                                                if attrs.is_empty() {
                                                    continue;
                                                }else {
                                                    for attr in attrs {
                                                        let path_segments = attr.path.segments;
                                                        let tokens = attr.tokens.into_iter();
                                                        for path_segment in path_segments {
                                                            let segment_ident = path_segment.ident;
                                                            if segment_ident == "route" {
                                                                for token in tokens.clone() {
                                                                    match token {
                                                                        TokenTree::Group(group)=>{
                                                                            let stream = group.stream();
                                                                            let mut route_params = vec![];
                                                                            for tag in stream.into_iter() {
                                                                                match tag {
                                                                                    TokenTree::Group(_) => {}
                                                                                    TokenTree::Ident(_) => {}
                                                                                    TokenTree::Punct(_) => {}
                                                                                    TokenTree::Literal(lit) => {
                                                                                        route_params.push(lit);
                                                                                    }
                                                                                }
                                                                            }
                                                                            if route_params.is_empty() || route_params.len() != 2 {
                                                                                panic!("路由参数无效，正确路由例子如： #[route(\"post\", \"/post\")]");
                                                                            }
                                                                            route_method = route_params[0].to_string().replace("\"","");
                                                                            route_url = route_params[1].to_string().replace("\"","");
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                //获取函数的参数信息
                                                let fn_params = func.sig.inputs;
                                                for fn_param in fn_params.into_iter() {
                                                    match fn_param {
                                                        FnArg::Receiver(_) => {}
                                                        FnArg::Typed(fn_param_type) => {
                                                            let pat = fn_param_type.pat;
                                                            let ty = fn_param_type.ty;
                                                            match *ty {
                                                                Type::Array(_) => {}
                                                                Type::BareFn(_) => {}
                                                                Type::Group(_) => {}
                                                                Type::ImplTrait(_) => {}
                                                                Type::Infer(_) => {}
                                                                Type::Macro(_) => {}
                                                                Type::Never(_) => {}
                                                                Type::Paren(_) => {}
                                                                Type::Path(path) => {
                                                                    has_request_ctx_params = path.path.segments[0].ident == "RequestCtx"
                                                                }
                                                                Type::Ptr(_) => {}
                                                                Type::Reference(_) => {}
                                                                Type::Slice(_) => {}
                                                                Type::TraitObject(_) => {}
                                                                Type::Tuple(_) => {}
                                                                Type::Verbatim(_) => {}
                                                                _ => {}
                                                            }
                                                            match *pat {
                                                                Pat::Box(_) => {}
                                                                Pat::Ident(_) => {

                                                                }
                                                                Pat::Lit(_) => {}
                                                                Pat::Macro(_) => {}
                                                                Pat::Or(_) => {}
                                                                Pat::Path(_) => {}
                                                                Pat::Range(_) => {}
                                                                Pat::Reference(_) => {}
                                                                Pat::Rest(_) => {}
                                                                Pat::Slice(_) => {}
                                                                Pat::Struct(_) => {}
                                                                Pat::Tuple(_) => {}
                                                                Pat::TupleStruct(_tuple_struct) => {
                                                                    let mut segments = vec![];
                                                                    for segment in _tuple_struct.path.segments {
                                                                        segments.push(segment.ident.to_string())
                                                                    }
                                                                    let mut i = 0;
                                                                    let mut fn_param_type = String::from("");
                                                                    for segment in segments {
                                                                        if i != 0 {
                                                                            fn_param_type = fn_param_type + "::" + &*segment;
                                                                        }else {
                                                                            fn_param_type = fn_param_type + &*segment;
                                                                        }
                                                                    }
                                                                    if fn_param_type == "Query" ||
                                                                        fn_param_type == "Json" ||
                                                                        fn_param_type == "Form"{
                                                                        has_form_or_query_or_json = true;
                                                                    }
                                                                }
                                                                Pat::Type(_) => {}
                                                                Pat::Verbatim(_) => {}
                                                                Pat::Wild(_) => {}
                                                                _ => {}
                                                            }
                                                        }
                                                    }
                                                }

                                                //动态生成路由注册函数
                                                let mut handler_fn = String::from("");
                                                let file_name = entry.file_name().to_str().unwrap().to_string().replace(".rs","");
                                                handler_fn = file_name.clone() + "::" + &*mod_ident.to_string() + "::" + &*fn_ident.to_string() + HANDLER_PROXY_FN_SUFFIX;
                                                register_route_fn = register_route_fn + "  register_route(\"" + &*route_method + "\",\"" + &*route_url + "\"," + &*handler_fn + ");\r\n"
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }

                    }
                }
            }

            if !register_route_fn.is_empty() {
                register_route_fn = "pub fn register_routes(){\r\n".to_owned() + &register_route_fn + "\r\n}";
                let register_route_fn_token_stream = TokenStream::from_str(&*register_route_fn);
                if register_route_fn_token_stream.is_err() {
                    panic!("动态生成注册路由函数失败：{}",register_route_fn_token_stream.err().unwrap());
                }else {
                    let func = parse_macro_input!(input as ItemFn);
                    let register_route_fn_token_stream = register_route_fn_token_stream.unwrap();
                    let mut expanded = quote! {
                        #func
                    };

                    let mut call_register_routes_fn = TokenStream2::from_str("register_routes();\r\n").unwrap();
                    expanded = expanded.into_iter().map(|tt| {
                        match tt {
                            TokenTree::Group(ref g)
                            if g.delimiter() == proc_macro2::Delimiter::Brace => {

                                call_register_routes_fn.extend(g.stream());

                                TokenTree::Group(proc_macro2::Group::new(
                                    proc_macro2::Delimiter::Brace, call_register_routes_fn.clone()))
                            },
                            other => other,
                        }
                    }).collect();
                    TokenStream::from_iter(vec![expanded.into(),register_route_fn_token_stream])
                }
            }else {
                let func = parse_macro_input!(input as ItemFn);
                let expanded = quote! {
                    #func
                };
                expanded.into()
            }

        }

    }
}