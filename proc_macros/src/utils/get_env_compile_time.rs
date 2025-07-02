//CI_COMMIT_SHA

use proc_macro::TokenStream;
use quote::quote;

pub fn get_env_compile_time(input: TokenStream) -> TokenStream {
    let variable_name = input.to_string().replace("\"", "");
    let value = std::env::var(&variable_name)
        .unwrap_or(format!("Cannot found {variable_name} in compile time!"));
    quote! {
        #value
    }
    .into()
}
