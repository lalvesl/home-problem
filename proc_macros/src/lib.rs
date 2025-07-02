mod cacher;
mod exec;
mod memory_tracker;
mod utils;

use proc_macro::TokenStream;

#[proc_macro]
pub fn try_block(input: TokenStream) -> TokenStream {
    exec::try_block::try_block(input)
}

#[proc_macro]
pub fn async_try_block(input: TokenStream) -> TokenStream {
    exec::async_try_block::async_try_block(input)
}

#[proc_macro]
pub fn enable_memory_tracker(_input: TokenStream) -> TokenStream {
    memory_tracker::enable::enable()
}

#[proc_macro_attribute]
pub fn disk_cache(attr: TokenStream, item: TokenStream) -> TokenStream {
    cacher::disk::disk(attr, item)
}

#[proc_macro_attribute]
pub fn db_cache(attr: TokenStream, item: TokenStream) -> TokenStream {
    cacher::db::db(attr, item)
}

#[proc_macro]
pub fn get_env_compile_time(input: TokenStream) -> TokenStream {
    utils::get_env_compile_time::get_env_compile_time(input)
}
