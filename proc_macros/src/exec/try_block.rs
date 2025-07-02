use proc_macro::TokenStream;

pub fn try_block(input: TokenStream) -> TokenStream {
    format!("(||->anyhow::Result<()>{{{input}}})()")
        .parse()
        .unwrap()
}
