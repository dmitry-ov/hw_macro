use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{Expr, Result};


/*
Процедурный макрос, принимающий набор строковых литералов - имён функций.
Макрос должен возвращать кортеж из возвращаемых значений тех функций,
в именах которых чётное количество символов.
Число функций может быть произвольным.
Пример:
let (fo_result, fooo_result) = my_macro!(""fo"", ""foo"", ""fooo"");

 */

#[proc_macro]
pub fn my_proc_macro(input: TokenStream) -> TokenStream {
     let tokens: proc_macro2::TokenStream = syn::parse_str(&input.to_string()).unwrap(); //  .split("").unwrap(); // expect("`my_proc_macro` macro expects only functions");

     let clean_tokens: Vec<String> = tokens.into_iter()
         .filter(|token| token.to_string().len() %2 ==0)
         .map(|token| token.to_string().trim_matches('\"').to_owned())
         .collect();

    quote::quote! {
            format_ident!((#clean_tokens), *)
    }.into()

    // TokenStream::default()
}
