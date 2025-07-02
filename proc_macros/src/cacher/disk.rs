use gutils::{regex_macro::regex, sha256::digest, traits::string::ReString};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use syn::{parse_macro_input, ItemFn};

pub fn disk(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let func_name = &input.sig.ident;
    let func_body = &input.block;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;
    let folder = "../../.cache";

    let arguments = input
        .sig
        .inputs
        .iter()
        .map(|arg| arg.into_token_stream().to_string())
        .collect::<Vec<String>>();

    let arguments_name = arguments
        .iter()
        .map(|arg| arg.replace_re(regex!(r"\s.*"), ""))
        .collect::<Vec<String>>();

    let arguments_stringfy: TokenStream2 = format!(
        "[{}].join(\"_\")",
        arguments_name[1..]
            .iter()
            .map(|arg| format!("{}.to_string()", arg))
            .collect::<Vec<String>>()
            .join(", ")
    )
    .parse()
    .unwrap();

    let hash_name = format!(
        "{}_{}",
        func_name,
        digest(arguments_name.join("_")).split_off(10)
    );

    let output_type: TokenStream2 = output
        .to_token_stream()
        .to_string()
        .replace_re(regex!("->"), "")
        .parse()
        .unwrap();

    let gen = quote::quote! {
        use tokio::fs;
        use std::path::Path;
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use gutils::sha256::digest;

        pub async fn #func_name(#inputs) #output {
            let cache_folder = Path::new(#folder);
            if !cache_folder.exists() {
                fs::create_dir(&cache_folder).await.unwrap();
            }

            let sub_cache_folder = cache_folder.join(#hash_name);
            if !sub_cache_folder.exists() {
                fs::create_dir(&sub_cache_folder).await.unwrap();
            }

            let cache_file = sub_cache_folder.join(format!("{}.json", digest(#arguments_stringfy)));
            let path = Path::new(&cache_file);

            if path.exists() {
                if let Ok(mut file) = fs::File::open(&cache_file).await {
                    let mut contents = String::new();
                    if file.read_to_string(&mut contents).await.is_ok() {
                        if let Ok(result) = serde_json::from_str::<_>(&contents) {
                            return Ok(result);
                        }
                    }
                }
            }

            let result: #output_type = (move || async move #func_body)().await;
            match result.as_ref() {
                Ok(data) => if let Ok(json) = serde_json::to_string(&data) {
                    if let Ok(mut file) = tokio::fs::File::create(&cache_file).await {
                        let _ = file.write_all(json.as_bytes()).await;
                    }
                }
                _=>(),
            }
            result
        }
    };

    gen.into()
}
