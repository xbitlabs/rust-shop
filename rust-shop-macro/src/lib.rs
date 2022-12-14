extern crate core;

use proc_macro::{Span, TokenStream};

use std::fs::File;
use std::io::Read;
use std::iter::FromIterator;
use std::path::PathBuf;
use std::str::FromStr;

use std::env;

use proc_macro2::TokenTree;

use quote::{quote, ToTokens};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};

use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, FnArg, GenericArgument, Ident, Item, ItemFn, Pat,
    PatTuple, PatType, Path, PathArguments, Type, TypePath,
};

use walkdir::WalkDir;

fn is_option_f32_type(type_name: &String) -> bool {
    return type_name == "Option<f32>";
}

fn is_option_f64_type(type_name: &String) -> bool {
    return type_name == "Option<f64>";
}

fn is_option_i8_type(type_name: &String) -> bool {
    return type_name == "Option<i8>";
}

fn is_option_u8_type(type_name: &String) -> bool {
    return type_name == "Option<u8>";
}

fn is_option_i16_type(type_name: &String) -> bool {
    return type_name == "Option<i16>";
}

fn is_option_u16_type(type_name: &String) -> bool {
    return type_name == "Option<u16>";
}

fn is_option_i32_type(type_name: &String) -> bool {
    return type_name == "Option<i32>";
}

fn is_option_u32_type(type_name: &String) -> bool {
    return type_name == "Option<u32>";
}

fn is_option_i64_type(type_name: &String) -> bool {
    return type_name == "Option<i64>";
}

fn is_option_u64_type(type_name: &String) -> bool {
    return type_name == "Option<u64>";
}

fn is_option_i128_type(type_name: &String) -> bool {
    return type_name == "Option<i128>";
}

fn is_option_u128_type(type_name: &String) -> bool {
    return type_name == "Option<u128>";
}

fn is_option_isize_type(type_name: &String) -> bool {
    return type_name == "Option<isize>";
}

fn is_option_usize_type(type_name: &String) -> bool {
    return type_name == "Option<usize>";
}

fn is_option_bool_type(type_name: &String) -> bool {
    return type_name == "Option<bool>";
}

fn is_option_datetime_type(type_name: &String) -> bool {
    return type_name == "Option<NaiveDateTime>";
}

fn is_option_date_type(type_name: &String) -> bool {
    return type_name == "Option<NaiveDate>";
}

fn is_option_string_type(type_name: &String) -> bool {
    return type_name == "Option<String>";
}

fn is_datetime_type(type_name: &String) -> bool {
    return type_name == "NaiveDateTime";
}

fn is_string_type(type_name: &String) -> bool {
    return type_name == "String";
}

fn is_date_type(type_name: &String) -> bool {
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
                let type_name = ty.to_token_stream().to_string().replace(" ", "");
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
                    } else if is_option_f64_type(&type_name) {
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
                    } else if is_option_i8_type(&type_name) {
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
                    } else if is_option_u8_type(&type_name) {
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
                    } else if is_option_i16_type(&type_name) {
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
                    } else if is_option_u16_type(&type_name) {
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
                    } else if is_option_i32_type(&type_name) {
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
                    } else if is_option_u32_type(&type_name) {
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
                    } else if is_option_i64_type(&type_name) {
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
                    } else if is_option_u64_type(&type_name) {
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
                    } else if is_option_i128_type(&type_name) {
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
                    } else if is_option_u128_type(&type_name) {
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
                    } else if is_option_isize_type(&type_name) {
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
                    } else if is_option_usize_type(&type_name) {
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
                    } else if is_option_bool_type(&type_name) {
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
                    } else if is_option_datetime_type(&type_name) {
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
                    } else if is_option_date_type(&type_name) {
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
                    } else if is_option_string_type(&type_name) {
                        quote!(
                            let param = form_params.get(stringify!(#ident));
                            let mut #ident = None;
                            if param.is_some() {
                                #ident = Some(param.unwrap().to_string());
                            }
                        )
                    } else {
                        quote!()
                    }
                } else {
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
                    } else if is_date_type(&type_name) {
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
                    //???????????????
                    else if !is_string_type(&type_name) {
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
                    } else {
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

                    pub async fn parse(self,req:&mut RequestCtx) -> anyhow::Result<#ident> {
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
///??????
///
struct Args {
    vars: Vec<syn::Expr>,
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
            None => {
                return Err(syn::Error::new(
                    Span::call_site().into(),
                    "No HTTP Method was provided",
                ))
            }
        }
    }

    pub fn get_route(&self) -> syn::Result<syn::Expr> {
        match self.vars.get(1) {
            Some(var) => Ok(var.clone()),
            None => {
                return Err(syn::Error::new(
                    Span::call_site().into(),
                    "No Route was provided",
                ))
            }
        }
    }
}

///handler?????????????????????
const HANDLER_PROXY_FN_SUFFIX: &str = "_handler_proxy";

///handler?????????
#[derive(Debug)]
struct HandlerParam {
    ///????????????????????????RequestParam,Json,Query???
    pub param_type: String,
    ///??????????????????RequestParam(id):RequestParam<u32>????????????id???????????????
    pub param_name: String,
    ///???????????????Option?????????RequestParam(id):RequestParam<Option<u32>>????????????????????????Option?????????????????????Option????????????????????????????????????u32,i64,String???
    pub param_option: String,
    ///??????Option????????????????????????RequestParam(id):RequestParam<Option<u32>>?????????????????????Option?????????????????????u32
    pub param_option_type: String,
}

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    //?????????????????????????????????????????????????????????
    let args = parse_macro_input!(args as Args);
    //?????????
    let func = parse_macro_input!(input as ItemFn);

    let idents = func.sig.inputs.iter().filter_map(|param| {
        if let syn::FnArg::Typed(pat_type) = param {
            if let syn::Pat::Ident(pat_ident) = *pat_type.pat.clone() {
                return Some(pat_ident.ident);
            }
        }

        None
    });
    let mut punctuated: Punctuated<syn::Ident, Comma> = Punctuated::new();
    idents.for_each(|ident| punctuated.push(ident));

    //????????????????????????
    let vis = func.vis.clone();
    //????????????
    let ident = func.sig.ident.clone();
    //?????????????????????get,post
    let method = args.get_method().unwrap();
    //????????????
    let route = args.get_route().unwrap();

    let fn_name = ident.to_string();
    let handler_proxy_name = fn_name.clone() + HANDLER_PROXY_FN_SUFFIX;

    let mut params = vec![];

    for arg in func.sig.inputs.iter() {
        //RequestParam,user,String
        //????????????????????????RequestParam,Json,Query???
        let mut handler_param_type = String::from("");
        //??????????????????RequestParam(id):RequestParam<u32>????????????id???????????????
        let mut handler_param_name = String::from("");
        //???????????????Option?????????RequestParam(id):RequestParam<Option<u32>>????????????????????????Option?????????????????????Option????????????????????????????????????u32,i64,String???
        let mut handler_param_option = String::from("");
        //??????Option????????????????????????RequestParam(id):RequestParam<Option<u32>>?????????????????????Option?????????????????????u32
        let mut handler_param_option_type = String::from("");
        if let FnArg::Typed(pat_type) = arg {
            match &*pat_type.pat {
                Pat::Ident(pat_ident) => {
                    handler_param_name = pat_ident.ident.to_string();
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
                Type::Reference(reference) => {
                    let ref_ele = reference.elem;
                    let is_mut_ref = reference.mutability.is_some();
                    let mut_str = if is_mut_ref { "mut " } else { "" };
                    match *ref_ele {
                        Type::Path(ref_type_path) => {
                            handler_param_type = String::from("&")
                                + mut_str
                                + &*ref_type_path.path.segments[0].ident.to_string();
                            //panic!("{}",handler_param_type);
                            let arguments = ref_type_path.path.segments.first().unwrap();
                            let path_args = &arguments.arguments;
                            if !path_args.is_empty() {
                                match path_args {
                                    PathArguments::AngleBracketed(angle_bracketed) => {
                                        let arg = angle_bracketed.args.first().unwrap();
                                        match arg {
                                            GenericArgument::Type(arg_type) => match arg_type {
                                                Type::Path(arg_path) => {
                                                    handler_param_type = handler_param_type
                                                        + "<"
                                                        + &*arg_path
                                                            .path
                                                            .segments
                                                            .first()
                                                            .unwrap()
                                                            .ident
                                                            .to_string()
                                                        + ">";
                                                }
                                                _ => {}
                                            },
                                            _ => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Type::Path(path) => {
                    handler_param_type = path.path.segments[0].ident.to_string();
                    let arguments = path.path.segments.first().unwrap();
                    let path_args = &arguments.arguments;
                    match path_args {
                        PathArguments::AngleBracketed(angle_bracketed) => {
                            let arg = angle_bracketed.args.first().unwrap();
                            match arg {
                                GenericArgument::Type(arg_type) => match arg_type {
                                    Type::Path(arg_path) => {
                                        handler_param_option = arg_path
                                            .path
                                            .segments
                                            .first()
                                            .unwrap()
                                            .ident
                                            .to_string();
                                        let path_args =
                                            &arg_path.path.segments.first().unwrap().arguments;
                                        match path_args {
                                            PathArguments::AngleBracketed(angle_bracketed) => {
                                                let arg = angle_bracketed.args.first().unwrap();
                                                match arg {
                                                    GenericArgument::Type(arg_type) => {
                                                        match arg_type {
                                                            Type::Path(arg_path) => {
                                                                handler_param_option_type =
                                                                    arg_path
                                                                        .path
                                                                        .segments
                                                                        .first()
                                                                        .unwrap()
                                                                        .ident
                                                                        .to_string();
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
                                },
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            params.push(HandlerParam {
                param_type: handler_param_type,
                param_name: handler_param_name,
                param_option: handler_param_option,
                param_option_type: handler_param_option_type,
            });
        }
    }
    //panic!("{:?}",params);
    if params.is_empty() {
        panic!("handler???????????????0????????????????????????RequestCtx??????");
    }

    let mut handler_proxy_fn_body = String::from("");
    let mut original_fn_inputs = vec![];

    let mut inject_sql_command_executor = false;
    let mut sql_command_executor_param_name = String::from("");
    for param in params {
        if param.param_type == "&mut SqlCommandExecutor" {
            inject_sql_command_executor = true;
            sql_command_executor_param_name = param.param_name.clone();
            original_fn_inputs.push(String::from("&mut ") + &*param.param_name.clone());
        } else if param.param_type == "&mut RequestCtx" {
            original_fn_inputs.push(String::from("ctx"));
        } else if param.param_type == "Json" {
            handler_proxy_fn_body = handler_proxy_fn_body
                + "  let "
                + &*param.param_name
                + " = Json::from_request(ctx).await?;\r\n";
            original_fn_inputs.push(param.param_name.clone());
        } else if param.param_type == "Form" {
            handler_proxy_fn_body = handler_proxy_fn_body
                + "  let "
                + &*param.param_name
                + " = Form::from_request(ctx).await?;\r\n";
            original_fn_inputs.push(param.param_name.clone());
        } else if param.param_type == "Query" {
            handler_proxy_fn_body = handler_proxy_fn_body
                + "  let "
                + &*param.param_name
                + " = Query::from_request(ctx).await?;\r\n";
            original_fn_inputs.push(param.param_name.clone());
        } else if param.param_type == "Header" {
            if param.param_option.eq("Option") {
                let header_tmp_var = param.param_name.clone() + "_tmp_var";
                handler_proxy_fn_body = handler_proxy_fn_body
                    + &*format!(
                    "let mut {0}:Header<Option<String>> = Header(None);\r\n
                    let {1} = ctx.headers.get(\"{0}\");\r\n
                    if {1}.is_some() {{\r\n
                        let {1} = {1}.unwrap();\r\n\
                        let {1} = {1}.to_str();
                        if {1}.is_ok() {{\r\n
                            {0} = Header(Some({1}.unwrap().to_string()));\r\n
                        }}\r\n
                    }}\r\n",
                    param.param_name.clone(),
                    header_tmp_var
                );
            } else {
                let msg = format!("header '{}' is None", param.param_name);
                let header_tmp_var_1 = param.param_name.clone() + "_tmp_var_1";
                let header_tmp_var_2 = param.param_name.clone() + "_tmp_var_2";
                handler_proxy_fn_body = handler_proxy_fn_body
                    + &*format!(
                    "let mut {0}:Option<Header<String>> = None;  \r\n
                    let {1} = ctx.headers.get(\"{3}\");          \r\n
                    if {1}.is_none() {{                           \r\n
                        return Err(anyhow!(\"{2}\"));            \r\n
                    }}else{{                                       \r\n
                        let {1} = {1}.unwrap();                  \r\n\
                        let {1} = {1}.to_str();                  \r\n
                        if {1}.is_err() {{                       \r\n
                            return Err(anyhow!(\"{2}\"));        \r\n
                        }}else {{                                  \r\n
                            {0} = Some(Header({1}.unwrap().to_string()));    \r\n
                        }}                                        \r\n
                    }}                                            \r\n
                    let {3}:Header<String> = {0}.unwrap();       \r\n",
                    header_tmp_var_1, header_tmp_var_2, msg, param.param_name
                );
            }
            original_fn_inputs.push(param.param_name.clone());
        } else if param.param_type == "PathVariable" {
            //panic!("{}",param.2);
            if param.param_option.eq("Option") {
                let header_tmp_var = param.param_name.clone() + "_tmp_var";
                let msg = format!("PathVariable '{}' is invalid", param.param_name.clone());
                handler_proxy_fn_body = handler_proxy_fn_body
                    + &*format!(
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
                    param.param_name, header_tmp_var, param.param_option_type, msg
                );
            } else {
                let msg_none = format!("PathVariable '{}' is None", param.param_name);
                let msg_invalid = format!("PathVariable '{}' is invalid", param.param_name);
                let header_tmp_var = param.param_name.clone() + "_tmp_var";
                handler_proxy_fn_body = handler_proxy_fn_body
                    + &*format!(
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
                    let {0} = {0}.unwrap();\r\n",
                    param.param_name,
                    header_tmp_var,
                    msg_none,
                    msg_invalid,
                    param.param_option
                );
            }
            original_fn_inputs.push(param.param_name.clone());
        } else if param.param_type == "RequestParam" {
            if param.param_option.eq("Option") {
                let header_tmp_var = param.param_name.clone() + "_tmp_var";
                let msg = format!("RequestParam '{}' is invalid", param.param_name.clone());
                handler_proxy_fn_body = handler_proxy_fn_body
                    + &*format!(
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
                    param.param_name, header_tmp_var, param.param_option_type, msg
                );
            } else {
                let msg_none = format!("RequestParam '{}' is None", param.param_name);
                let msg_invalid = format!("RequestParam '{}' is invalid", param.param_name);

                let header_tmp_var = param.param_name.clone() + "_tmp_var";
                handler_proxy_fn_body = handler_proxy_fn_body
                    + &*format!(
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
                    let {0} = {0}.unwrap();\r\n",
                    param.param_name,
                    header_tmp_var,
                    msg_none,
                    msg_invalid,
                    param.param_option
                );
            }
            original_fn_inputs.push(param.param_name.clone());
        } else if param.param_type == "Multipart" {
            handler_proxy_fn_body = handler_proxy_fn_body
                + "  let "
                + &*param.param_name
                + " = Multipart::from_request(ctx).await?;\r\n";
            original_fn_inputs.push(param.param_name.clone());
        } else {
            panic!("???????????????????????????{}", param.param_type);
        }
    }
    //panic!("{:?}",original_fn_inputs);
    let mut inputs = String::from("");
    let mut i = 0;
    for original_fn_input in original_fn_inputs {
        if i == 0 {
            inputs = inputs + &*original_fn_input;
        } else {
            inputs = inputs + "," + &*original_fn_input;
        }
        i = i + 1;
    }
    let original_fn_name = ident.to_string();

    let mut sql_command_executor_inject_code = String::from("");
    let mut tran_commit_rollback_code = String::from("");
    let with_tran = sql_command_executor_param_name == "sql_exe_with_tran";
    let mut handle_result = "let handler_result = ".to_string()
        + &*original_fn_name
        + "("
        + &*inputs
        + ").await?;\r\n"
        + "Ok(handler_result.into_response())\r\n";
    if inject_sql_command_executor {
        sql_command_executor_inject_code = "let mut pool_state: Option<&State<Pool<MySql>>> = None;
            unsafe{ pool_state = APP_EXTENSIONS.get(); }
            let pool = pool_state.unwrap().get_ref();
        "
        .to_string();
        if with_tran {
            sql_command_executor_inject_code = sql_command_executor_inject_code
                + &*format!(
                    "
            let tran = pool.begin().await?;
            let mut tran_manager = TransactionManager::new(tran);
            let mut {0} = SqlCommandExecutor::WithTransaction(&mut tran_manager);",
                    sql_command_executor_param_name
                );

            handle_result = "let handler_result = ".to_string()
                + &*original_fn_name
                + "("
                + &*inputs
                + ").await;\r\n";

            tran_commit_rollback_code = "return if handler_result.is_err() {{
                    println!(\"{}\",\"????????????\");
                    tran_manager.rollback().await?;
                    Err(handler_result.err().unwrap())
                }} else {{
                    println!(\"{}\",\"????????????\");
                    tran_manager.commit().await?;
                    Ok(handler_result.unwrap().into_response())
                }}"
            .to_string();
        } else {
            sql_command_executor_inject_code = sql_command_executor_inject_code
                + &*format!(
                    "
            let mut {0} = SqlCommandExecutor::WithoutTransaction(pool);",
                    sql_command_executor_param_name
                );
        }
    }

    let handler_proxy_fn = String::from("pub async fn ")
        + &*handler_proxy_name
        + "(mut req_ctx:RequestCtx)->anyhow::Result<Response>{\r\n"
        + "let ctx = &mut req_ctx;"
        + &*sql_command_executor_inject_code
        + &*handler_proxy_fn_body
        + &*handle_result
        + &*tran_commit_rollback_code
        + "}\r\n";
    //panic!("{:#?}",handler_proxy_fn);
    let handler_token_stream = TokenStream::from_str(handler_proxy_fn.as_str()).unwrap();

    let expanded = quote! {
        #func
    };
    TokenStream::from_iter(vec![expanded.into(), handler_token_stream])
}

///
/// ?????????????????????????????????
///
#[proc_macro_attribute]
pub fn scan_route(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut args = args.to_string();
    if args.is_empty() {
        panic!("?????????????????????controller????????????????????????:/src/controller");
    } else {
        let current_dir = env::current_dir();
        if current_dir.is_err() {
            panic!("????????????????????????");
        } else {
            args = args.replace("\"", "");
            let current_dir = current_dir
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                .replace("\\", "/");
            let mut path = String::from("");
            path = current_dir + &*args;
            let source_path = PathBuf::from(&path);
            if !source_path.exists() {
                panic!("??????????????????{}", path)
            }
            let mut register_route_fn = String::from("");
            for entry in WalkDir::new(path) {
                let entry = entry.unwrap();
                let file = entry.path().to_str().unwrap().to_string();
                if entry.path().is_file() && file.ends_with("_controller.rs") {
                    let mut file = File::open(file).expect("Unable to open file");

                    let mut src = String::new();
                    file.read_to_string(&mut src).expect("Unable to read file");

                    let syntax = syn::parse_file(&src).expect("Unable to parse file");
                    //println!("{:#?}", syntax);
                    for item in syntax.items {
                        match item {
                            Item::Mod(item_mod) => {
                                let mod_ident = item_mod.ident;
                                if item_mod.content.is_some() {
                                    //println!("{}",item_mod.content.unwrap().1.len());
                                    let mod_content_items = item_mod.content.unwrap().1;
                                    for mod_content_item in mod_content_items {
                                        match mod_content_item {
                                            Item::Fn(func) => {
                                                //??????????????????
                                                let attrs = func.attrs;
                                                let fn_ident = func.sig.ident;
                                                let mut route_method = String::from("");
                                                let mut route_url = String::from("");
                                                let mut has_request_ctx_params = false;
                                                let mut has_form_or_query_or_json = false;
                                                //????????????route???????????????
                                                if attrs.is_empty() {
                                                    continue;
                                                } else {
                                                    for attr in attrs {
                                                        let path_segments = attr.path.segments;
                                                        let tokens = attr.tokens.into_iter();
                                                        for path_segment in path_segments {
                                                            let segment_ident = path_segment.ident;
                                                            if segment_ident == "route" {
                                                                for token in tokens.clone() {
                                                                    match token {
                                                                        TokenTree::Group(group) => {
                                                                            let stream =
                                                                                group.stream();
                                                                            let mut route_params =
                                                                                vec![];
                                                                            for tag in
                                                                                stream.into_iter()
                                                                            {
                                                                                match tag {
                                                                                    TokenTree::Group(_) => {}
                                                                                    TokenTree::Ident(_) => {}
                                                                                    TokenTree::Punct(_) => {}
                                                                                    TokenTree::Literal(lit) => {
                                                                                        route_params.push(lit);
                                                                                    }
                                                                                }
                                                                            }
                                                                            if route_params
                                                                                .is_empty()
                                                                                || route_params
                                                                                    .len()
                                                                                    != 2
                                                                            {
                                                                                panic!("????????????????????????????????????????????? #[route(\"post\", \"/post\")]");
                                                                            }
                                                                            route_method =
                                                                                route_params[0]
                                                                                    .to_string()
                                                                                    .replace(
                                                                                        "\"", "",
                                                                                    );
                                                                            route_url =
                                                                                route_params[1]
                                                                                    .to_string()
                                                                                    .replace(
                                                                                        "\"", "",
                                                                                    );
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                //???????????????????????????
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
                                                                    has_request_ctx_params =
                                                                        path.path.segments[0].ident
                                                                            == "RequestCtx"
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
                                                                Pat::Ident(_) => {}
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
                                                                    for segment in
                                                                        _tuple_struct.path.segments
                                                                    {
                                                                        segments.push(
                                                                            segment
                                                                                .ident
                                                                                .to_string(),
                                                                        )
                                                                    }
                                                                    let mut i = 0;
                                                                    let mut fn_param_type =
                                                                        String::from("");
                                                                    for segment in segments {
                                                                        if i != 0 {
                                                                            fn_param_type =
                                                                                fn_param_type
                                                                                    + "::"
                                                                                    + &*segment;
                                                                        } else {
                                                                            fn_param_type =
                                                                                fn_param_type
                                                                                    + &*segment;
                                                                        }
                                                                    }
                                                                    if fn_param_type == "Query"
                                                                        || fn_param_type == "Json"
                                                                        || fn_param_type == "Form"
                                                                    {
                                                                        has_form_or_query_or_json =
                                                                            true;
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

                                                //??????????????????????????????
                                                let mut handler_fn = String::from("");
                                                let file_name = entry
                                                    .file_name()
                                                    .to_str()
                                                    .unwrap()
                                                    .to_string()
                                                    .replace(".rs", "");
                                                handler_fn = file_name.clone()
                                                    + "::"
                                                    + &*mod_ident.to_string()
                                                    + "::"
                                                    + &*fn_ident.to_string()
                                                    + HANDLER_PROXY_FN_SUFFIX;
                                                register_route_fn = register_route_fn
                                                    + "  register_route(\""
                                                    + &*route_method
                                                    + "\".to_string(),\""
                                                    + &*route_url
                                                    + "\".to_string(),"
                                                    + &*handler_fn
                                                    + ");\r\n"
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
                register_route_fn =
                    "pub fn register_routes(){\r\n".to_owned() + &register_route_fn + "\r\n}";
                let register_route_fn_token_stream = TokenStream::from_str(&*register_route_fn);
                if register_route_fn_token_stream.is_err() {
                    panic!(
                        "???????????????????????????????????????{}",
                        register_route_fn_token_stream.err().unwrap()
                    );
                } else {
                    let func = parse_macro_input!(input as ItemFn);
                    let register_route_fn_token_stream = register_route_fn_token_stream.unwrap();
                    let mut expanded = quote! {
                        #func
                    };

                    let mut call_register_routes_fn =
                        TokenStream2::from_str("register_routes();\r\n").unwrap();
                    expanded = expanded
                        .into_iter()
                        .map(|tt| match tt {
                            TokenTree::Group(ref g)
                                if g.delimiter() == proc_macro2::Delimiter::Brace =>
                            {
                                call_register_routes_fn.extend(g.stream());

                                TokenTree::Group(proc_macro2::Group::new(
                                    proc_macro2::Delimiter::Brace,
                                    call_register_routes_fn.clone(),
                                ))
                            }
                            other => other,
                        })
                        .collect();
                    TokenStream::from_iter(vec![expanded.into(), register_route_fn_token_stream])
                }
            } else {
                let func = parse_macro_input!(input as ItemFn);
                let expanded = quote! {
                    #func
                };
                expanded.into()
            }
        }
    }
}


use inflector::Inflector;
use proc_macro::{self};
use quote::{format_ident};

use syn::{Attribute, DataStruct, Field, FieldsNamed,
    Lit, LitStr, Meta, MetaNameValue,
};


#[proc_macro_derive(SqlxCrud, attributes(database, external_id, id))]
pub fn sqlx_crud_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, data, attrs, ..
    } = parse_macro_input!(input);
    match data {
        Data::Struct(DataStruct {
                         fields: Fields::Named(FieldsNamed { named, .. }),
                         ..
                     }) => {
            let config = Config::new(&attrs, &ident, &named);
            let static_model_schema = build_static_model_schema(&config);
            let sqlx_crud_impl = build_sqlx_crud_impl(&config);

            quote! {
                #static_model_schema
                #sqlx_crud_impl
            }
            .into()
        }
        _ => panic!("this derive macro only works on structs with named fields"),
    }
}

fn build_static_model_schema(config: &Config) -> TokenStream2 {
    let crate_name = &config.crate_name;
    let model_schema_ident = &config.model_schema_ident;
    let table_name = &config.table_name;

    let id_column = config.id_column_ident.to_string();
    let columns_len = config.named.iter().count();
    let columns = config
        .named
        .iter()
        .flat_map(|f| &f.ident)
        .map(|f| LitStr::new(format!("{}", f).as_str(), f.span()));

    let sql_queries = build_sql_queries(config);

    quote! {
        #[automatically_derived]
        static #model_schema_ident: #crate_name::schema::Metadata<'static, #columns_len> = #crate_name::schema::Metadata {
            table_name: #table_name,
            id_column: #id_column,
            columns: [#(#columns),*],
            #sql_queries
        };
    }
}

fn build_sql_queries(config: &Config) -> TokenStream2 {
    let table_name = config.quote_ident(&config.table_name);
    let id_column = format!(
        "{}.{}",
        &table_name,
        config.quote_ident(&config.id_column_ident.to_string())
    );

    let insert_bind_cnt = config.named.iter().count();

    let insert_sql_binds = (0..insert_bind_cnt)
        .map(|_| "?")
        .collect::<Vec<_>>()
        .join(", ");

    let update_sql_binds = config
        .named
        .iter()
        .flat_map(|f| &f.ident)
        .filter(|i| *i != &config.id_column_ident)
        .map(|i| format!("{} = ?", config.quote_ident(&i.to_string())))
        .collect::<Vec<_>>()
        .join(", ");

    let insert_column_list = config
        .named
        .iter()
        .flat_map(|f| &f.ident)
        .filter(|i| config.external_id || (*i != &config.id_column_ident || *i == &config.id_column_ident))
        .map(|i| config.quote_ident(&i.to_string()))
        .collect::<Vec<_>>()
        .join(", ");
    let column_list = config
        .named
        .iter()
        .flat_map(|f| &f.ident)
        .map(|i| format!("{}.{}", &table_name, config.quote_ident(&i.to_string())))
        .collect::<Vec<_>>()
        .join(", ");

    let select_sql = format!("SELECT {} FROM {}", column_list, table_name);
    let select_by_id_sql = format!(
        "SELECT {} FROM {} WHERE {} = ? ",
        column_list, table_name, id_column
    );
    let insert_sql = format!(
        "INSERT INTO {} ({}) VALUES ({}) ",
        table_name, insert_column_list, insert_sql_binds
    );
    let update_by_id_sql = format!(
        "UPDATE {} SET {} WHERE {} = ? ",
        table_name, update_sql_binds, id_column
    );
    let delete_by_id_sql = format!("DELETE FROM {} WHERE {} = ?", table_name, id_column);

    quote! {
        select_sql: #select_sql,
        select_by_id_sql: #select_by_id_sql,
        insert_sql: #insert_sql,
        update_by_id_sql: #update_by_id_sql,
        delete_by_id_sql: #delete_by_id_sql,
    }
}

fn build_sqlx_crud_impl(config: &Config) -> TokenStream2 {
    let crate_name = &config.crate_name;
    let ident = &config.ident;
    let model_schema_ident = &config.model_schema_ident;
    let id_column_ident = &config.id_column_ident;
    let id_ty = config
        .named
        .iter()
        .find(|f| f.ident.as_ref() == Some(id_column_ident))
        .map(|f| &f.ty)
        .expect("the id type");

    let insert_binds = config
        .named
        .iter()
        .flat_map(|f| &f.ident)
        .map(|i| quote! { .bind(&self.#i) });
    let update_binds = config
        .named
        .iter()
        .flat_map(|f| &f.ident)
        .filter(|i| *i != id_column_ident)
        .map(|i| quote! { .bind(&self.#i) });

    let db_ty = config.db_ty.sqlx_db();

    quote! {
        #[automatically_derived]
        impl #crate_name::traits::Schema for #ident {
            type Id = #id_ty;

            fn table_name() -> &'static str {
                #model_schema_ident.table_name
            }

            fn id(&self) -> Self::Id {
                self.#id_column_ident
            }

            fn id_column() -> &'static str {
                #model_schema_ident.id_column
            }

            fn columns() -> &'static [&'static str] {
                &#model_schema_ident.columns
            }

            fn select_sql() -> &'static str {
                #model_schema_ident.select_sql
            }

            fn select_by_id_sql() -> &'static str {
                #model_schema_ident.select_by_id_sql
            }

            fn insert_sql() -> &'static str {
                #model_schema_ident.insert_sql
            }

            fn update_by_id_sql() -> &'static str {
                #model_schema_ident.update_by_id_sql
            }

            fn delete_by_id_sql() -> &'static str {
                #model_schema_ident.delete_by_id_sql
            }
        }

        #[automatically_derived]
        impl<'e> #crate_name::traits::Crud<'e> for #ident {
            fn insert_binds(
                &'e self,
                query: ::sqlx::query::QueryAs<'e, MySql, Self, MySqlArguments>
            ) -> ::sqlx::query::QueryAs<'e, MySql, Self, MySqlArguments> {
                query
                    #(#insert_binds)*
            }

            fn update_binds(
                &'e self,
                query: ::sqlx::query::QueryAs<'e, MySql, Self, MySqlArguments>
            ) -> ::sqlx::query::QueryAs<'e, MySql, Self, MySqlArguments> {
                query
                    #(#update_binds)*
                    .bind(&self.#id_column_ident)
            }
        }
    }
}

#[allow(dead_code)] // Usage in quote macros aren't flagged as used
struct Config<'a> {
    ident: &'a Ident,
    named: &'a Punctuated<Field, Comma>,
    crate_name: TokenStream2,
    db_ty: DbType,
    model_schema_ident: Ident,
    table_name: String,
    id_column_ident: Ident,
    external_id: bool,
}

impl<'a> Config<'a> {
    fn new(attrs: &[Attribute], ident: &'a Ident, named: &'a Punctuated<Field, Comma>) -> Self {
        let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
        let is_doctest = std::env::vars()
            .any(|(k, _)| k == "UNSTABLE_RUSTDOC_TEST_LINE" || k == "UNSTABLE_RUSTDOC_TEST_PATH");
        let crate_name = if !is_doctest && crate_name == "rust_shop_core::db" {
            quote! { crate }
        } else {
            quote! { rust_shop_core::db }
        };

        let db_ty = DbType::new(attrs);

        let model_schema_ident =
            format_ident!("{}_SCHEMA", ident.to_string().to_screaming_snake_case());

        let table_name = ident.to_string().to_snake_case();

        // Search for a field with the #[id] attribute
        let id_attr = &named
            .iter()
            .find(|f| f.attrs.iter().any(|a| a.path.is_ident("id")))
            .and_then(|f| f.ident.as_ref());
        // Otherwise default to the first field as the "id" column
        let id_column_ident = id_attr
            .unwrap_or_else(|| {
                named
                    .iter()
                    .flat_map(|f| &f.ident)
                    .next()
                    .expect("the first field")
            })
            .clone();

        let external_id = attrs.iter().any(|a| a.path.is_ident("external_id"));

        Self {
            ident,
            named,
            crate_name,
            db_ty,
            model_schema_ident,
            table_name,
            id_column_ident,
            external_id,
        }
    }

    fn quote_ident(&self, ident: &str) -> String {
        self.db_ty.quote_ident(ident)
    }
}

enum DbType {
    Any,
    Mssql,
    MySql,
    Postgres,
    Sqlite,
}

impl From<&str> for DbType {
    fn from(db_type: &str) -> Self {
        match db_type {
            "Any" => Self::Any,
            "Mssql" => Self::Mssql,
            "MySql" => Self::MySql,
            "Postgres" => Self::Postgres,
            "Sqlite" => Self::Sqlite,
            _ => panic!("unknown #[database] type {}", db_type),
        }
    }
}

impl DbType {
    fn new(attrs: &[Attribute]) -> Self {
        match attrs
            .iter()
            .find(|a| a.path.is_ident("database"))
            .map(|a| a.parse_meta())
        {
            Some(Ok(Meta::NameValue(MetaNameValue {
                                        lit: Lit::Str(s), ..
                                    }))) => DbType::from(&*s.value()),
            _ => Self::Sqlite,
        }
    }

    fn sqlx_db(&self) -> TokenStream2 {
        match self {
            Self::Any => quote! { ::sqlx::Any },
            Self::Mssql => quote! { ::sqlx::Mssql },
            Self::MySql => quote! { ::sqlx::MySql },
            Self::Postgres => quote! { ::sqlx::Postgres },
            Self::Sqlite => quote! { ::sqlx::Sqlite },
        }
    }

    fn quote_ident(&self, ident: &str) -> String {
        match self {
            Self::Any => format!(r#"{}"#, &ident),
            Self::Mssql => format!(r#"{}"#, &ident),
            Self::MySql => format!("`{}`", &ident),
            Self::Postgres => format!(r#"{}"#, &ident),
            Self::Sqlite => format!(r#"{}"#, &ident),
        }
    }
}
