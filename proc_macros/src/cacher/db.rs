use darling::FromMeta;
use gutils::{regex_macro::regex, traits::string::ReString};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, ToTokens};
use syn::{parse_macro_input, FnArg, ItemFn};

#[allow(unused)]
#[derive(FromMeta)]
struct MacroArgs {
    pub revalidate: Option<String>,
}

pub fn db(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let func_name = &input.sig.ident;
    let func_name_str = func_name.to_string();
    let fn_args = &input.sig.inputs;
    let func_body = &input.block;
    let inputs = &input.sig.inputs;
    let output = &input.sig.output;

    let args_to_vec = if !fn_args.is_empty() {
        let args_to_string = fn_args
            .iter()
            .filter(|arg| matches!(arg, FnArg::Typed(_arg)))
            .map(|variable| {
                let variable_string = match variable {
                    FnArg::Typed(arg) => arg
                        .to_token_stream()
                        .to_string()
                        .replace_re(regex!(r"\s.*"), ""),
                    _ => panic!("Impossible"),
                };
                let variable = format_ident!("{}", variable_string.to_string());
                quote::quote! {
                    format!("{} = {:?}, ", #variable_string, #variable).as_str()
                }
            })
            .collect::<Vec<_>>();
        quote::quote! {
            [#(#args_to_string),*].join(",").as_str()
        }
    } else {
        quote::quote! {""}
    };

    let key = quote::quote! {
        digest(&(#func_name_str.to_string() + #args_to_vec))
    };

    let output_type: TokenStream2 = output
        .to_token_stream()
        .to_string()
        .replace_re(regex!("->"), "")
        .parse()
        .unwrap();

    let output_type_unwraped: TokenStream2 = output_type
        .to_string()
        .replace_re_all(regex!(r"(Result\W+<)|(>$)"), "")
        .parse()
        .unwrap();

    let gen = quote::quote! {
        pub async fn #func_name(#inputs) #output {
            use super::*;
            use gutils::{
                bincode,
                compress,
                chrono::{NaiveDateTime, Utc},
                db::cdp::cdp,
                sea_orm::{
                    ActiveModelTrait,
                    ActiveValue,
                    ColumnTrait,
                    EntityTrait,
                    ModelTrait,
                    QueryFilter,
                },
                schema::cdp::key_value,
                sha256::digest,
            };

            let slugs = #func_name_str;

            let key = #key;

            let now:NaiveDateTime = Utc::now().naive_utc();

            let db = &cdp().await.unwrap();

            match key_value::Entity::find()
                .filter(key_value::Column::Key.eq(&key))
                .one(db)
                .await
            {
                Ok(data) => match data {
                    Some(data) => {
                        let data = compress::decompress(&data.data);
                        let value: #output_type_unwraped = match bincode::deserialize(&data) {
                            Ok(value) => value,
                            Err(e) => {
                                let value: #output_type = (move || async move #func_body)().await;
                                return match value {
                                    Ok(value)=>{
                                        let data: Vec<u8> = bincode::serialize(&value).unwrap();
                                        let data = compress::compress(&data);
                                        let data = key_value::Model {
                                            key: key.to_string(),
                                            data,
                                            slugs: slugs.to_string(),
                                            created_at: now,
                                            expired_at: None,
                                            ..Default::default()
                                        };

                                        let data: key_value::ActiveModel = data.into();
                                        let _ = data.clone().insert(db).await;
                                        let _ = data.update(db).await;

                                        Ok(value)
                                    }
                                    Err(e)=>{
                                        println!(
                                            "Error on processs function '{}' inside cache, arguments: '{:?}', error: '{:?}'",
                                            #func_name_str,
                                            #args_to_vec,
                                            e
                                        );
                                        Err(e)
                                    }
                                }
                            },
                        };
                        return Ok(value);
                    }
                    None => {
                        let value: #output_type = (move || async move #func_body)().await;
                        return match value {
                            Ok(value)=>{
                                let data: Vec<u8> = bincode::serialize(&value).unwrap();
                                let data = compress::compress(&data);
                                let data = key_value::Model {
                                    key: key.to_string(),
                                    data,
                                    slugs: slugs.to_string(),
                                    created_at: now,
                                    expired_at: None,
                                    ..Default::default()
                                };

                                let data: key_value::ActiveModel = data.into();
                                let _ = data.clone().insert(db).await;
                                let _ = data.update(db).await;

                                Ok(value)
                            }
                            Err(e)=>{
                                println!(
                                    "Error on processs function '{}' inside cache, arguments: '{:?}', error: '{:?}'",
                                    #func_name_str,
                                    #args_to_vec,
                                    e
                                );
                                Err(e)
                            }
                        }
                    }
                },
                Err(e) => {
                    let value: #output_type = (move || async move #func_body)().await;
                    return match value {
                        Ok(value)=>{
                            let data: Vec<u8> = bincode::serialize(&value).unwrap();
                            let data = compress::compress(&data);
                            let data = key_value::Model {
                                key: key.to_string(),
                                data,
                                slugs: slugs.to_string(),
                                created_at: now,
                                expired_at: None,
                                ..Default::default()
                            };

                            let data: key_value::ActiveModel = data.into();
                            let _ = data.clone().insert(db).await;
                            let _ = data.update(db).await;

                            Ok(value)
                        }
                        Err(e)=>{
                            println!(
                                "Error on processs function '{}' inside cache, arguments: '{:?}', error: '{:?}'",
                                #func_name_str,
                                #args_to_vec,
                                e
                            );
                            Err(e)
                        }
                    }
                }
            }
        }
    };

    gen.into()
}
