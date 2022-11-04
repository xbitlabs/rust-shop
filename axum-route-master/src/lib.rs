#![feature(exact_size_is_empty)]

use proc_macro::{TokenStream, Span};
use std::alloc::System;
use syn::{parse_macro_input, ItemFn, FnArg, Type, TypePath, Path};
use syn::punctuated::Punctuated;
use syn::parse::{Parse, ParseStream};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::parse_quote;

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
