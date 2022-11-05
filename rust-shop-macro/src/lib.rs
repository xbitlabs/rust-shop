
use proc_macro::{Span, TokenStream};
use std::any::{Any, TypeId};
use std::iter::FromIterator;
use chrono::NaiveDateTime;
use syn::{Data, DeriveInput, Fields, FnArg, Ident, parse_macro_input, Type,ItemFn, TypePath, Path};
use syn::__private::TokenStream2;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use thiserror::Error;
use std::alloc::System;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::parse_quote;

#[derive(Error, Debug)]
enum Error {
    #[error("field `{0}` required, but not set yet.")]
    FieldNoValue(String),
}
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

#[proc_macro_attribute]
pub fn route(args: TokenStream, input: TokenStream) -> TokenStream {
    //宏传入的参数，从里面提取请求方法及路径
    let args = parse_macro_input!(args as Args);
    //方法体
    let func = parse_macro_input!(input as ItemFn);
    //println!("{:?}",func.sig.inputs.into_iter());
    // 1. Filter the params, so that only typed arguments remain
    // 2. Extract the ident (in case the pattern type is ident)

    let mut test = quote!{
        pub fn test111(){

        }
    };

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


    for arg in func.sig.inputs.iter() {
        if let FnArg::Typed(ty) = arg {
            let ty = ty.ty.clone();
            match *ty {
                Type::Array(TypeArray)=>{
                    test = quote!{
                        pub fn testTypeArray(){

                        }
                    };
                }

                /// A bare function type: `fn(usize) -> bool`.
                Type::BareFn(TypeBareFn)=>{
                    test = quote!{
                        pub fn testTypeBareFn(){

                        }
                    };
                }

                /// A type contained within invisible delimiters.
                Type::Group(TypeGroup)=>{
                    test = quote!{
                        pub fn testTypeGroup(){

                        }
                    };
                }

                /// An `impl Bound1 + Bound2 + Bound3` type where `Bound` is a trait or
                /// a lifetime.
                Type::ImplTrait(TypeImplTrait)=>{
                    test = quote!{
                        pub fn testTypeImplTrait(){

                        }
                    };
                }

                /// Indication that a type should be inferred by the compiler: `_`.
                Type::Infer(TypeInfer)=>{
                    test = quote!{
                        pub fn testTypeInfer(){

                        }
                    };
                }

                /// A macro in the type position.
                Type::Macro(TypeMacro)=>{
                    test = quote!{
                        pub fn testTypeMacro(){

                        }
                    };
                }

                /// The never type: `!`.
                Type::Never(TypeNever)=>{
                    test = quote!{
                        pub fn testTypeNever(){

                        }
                    };
                }

                /// A parenthesized type equivalent to the inner type.
                Type::Paren(TypeParen)=>{
                    test = quote!{
                        pub fn testTypeParen(){

                        }
                    };
                }

                /// A path like `std::slice::Iter`, optionally qualified with a
                /// self-type as in `<Vec<T> as SomeTrait>::Associated`.
                Type::Path(type_path)=>{
                    let mut  str:String = String::from("") ;

                    for s in  type_path.path.segments.iter() {
                        str = str + &s.ident.to_string();
                    }
                    test = quote!{
                        pub fn testTypePath(){
                            let str:String = #str;
                        }
                    };
                }

                /// A raw pointer type: `*const T` or `*mut T`.
                Type::Ptr(TypePtr)=>{
                    test = quote!{
                        pub fn testTypePtr(){

                        }
                    };
                }

                /// A reference type: `&'a T` or `&'a mut T`.
                Type::Reference(TypeReference)=>{
                    test = quote!{
                        pub fn testTypeReference(){

                        }
                    };
                }

                /// A dynamically sized slice type: `[T]`.
                Type::Slice(TypeSlice)=>{
                    test = quote!{
                        pub fn testTypeSlice(){

                        }
                    };
                }

                /// A trait object type `dyn Bound1 + Bound2 + Bound3` where `Bound` is a
                /// trait or a lifetime.
                Type::TraitObject(TypeTraitObject)=>{
                    test = quote!{
                        pub fn testTypeTraitObject(){

                        }
                    };
                }

                /// A tuple type: `(A, B, C, String)`.
                Type::Tuple(TypeTuple)=>{
                    test = quote!{
                pub fn testTypeTuple(){

                }
                    };
                }

                /// Tokens in type position not interpreted by Syn.
                Type::Verbatim(TokenStream)=>{
                    test = quote!{
                pub fn testTokenStream(){

                }
                    };
                }
                _ =>{
                    test = quote!{
                pub fn test2222__()-{

                }
            };
                }
            }

        }
    }

    //方法的访问修饰符
    let vis = func.vis.clone();
    //方法签名
    let ident = func.sig.ident.clone();
    //请求方法，如：get,post
    let method = args.get_method().unwrap();
    //请求路径
    let route = args.get_route().unwrap();

    let expanded = quote! {
        #[allow(non_camel_case_types)]
        #vis struct #ident;

        impl #ident {
            #vis fn route() -> axum::Router {
                #func

                axum::Router::new().route(#route, #method (#ident))
            }
            #test
            pub fn kkk(){

            }
        }
    };

    expanded.into()
}

