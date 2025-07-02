use proc_macro::TokenStream;

pub fn async_try_block(input: TokenStream) -> TokenStream {
    // let new_code = quote::quote! {
    //     (
    //         ||->impl std::future::Future<Output = gutils::anyhow::Result<()>> {
    //             async move {
    //                 #input
    //             }
    //         }
    //     )().await
    // };
    //
    // let _: TokenStream = new_code.into();
    input
}
