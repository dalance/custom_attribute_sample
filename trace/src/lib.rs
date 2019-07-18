extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Stmt};

//#[proc_macro_attribute]
//pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
//    let item = dbg!(item);
//    item
//}

//#[proc_macro_attribute]
//pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
//    let ret = item.clone();
//    let ast = parse_macro_input!(item as ItemFn);
//    dbg!(ast);
//    ret
//}

//#[proc_macro_attribute]
//pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
//    let mut ast = parse_macro_input!(item as ItemFn);
//
//    let new_stmt = quote! {
//        println!("trace");
//    };
//    let new_stmt: TokenStream = new_stmt.into();
//    let new_stmt = parse_macro_input!(new_stmt as Stmt);
//
//    ast.block.stmts.clear();
//    ast.block.stmts.push(new_stmt);
//
//    let gen = quote! {
//        #ast
//    };
//    gen.into()
//}

#[proc_macro_attribute]
pub fn trace(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemFn);
    let ident = &ast.ident;

    let enter = quote! {
        println!("Enter: {}", stringify!(#ident) );
    };
    let enter: TokenStream = enter.into();
    let enter = parse_macro_input!(enter as Stmt);

    let mut body = quote! {};
    for s in &ast.block.stmts {
        body = quote! {
            #body
            #s
        };
    }
    let body = quote! {
        let body = || { #body };
    };
    let body: TokenStream = body.into();
    let body = parse_macro_input!(body as Stmt);

    let exit = quote! {
        {
            let ret = body();
            println!("Exit: {}", stringify!(#ident) );
            ret
        }
    };
    let exit: TokenStream = exit.into();
    let exit = parse_macro_input!(exit as Stmt);

    ast.block.stmts.clear();
    ast.block.stmts.push(enter);
    ast.block.stmts.push(body);
    ast.block.stmts.push(exit);

    let gen = quote! {
        #ast
    };

    gen.into()
}
