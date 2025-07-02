extern crate proc_macro;
use proc_macro::TokenStream;
use quick_xml::events::Event;
use quote::quote;
use serde_json::{json, Value};
use std::collections::HashMap;
use syn::{parse_macro_input, LitStr};

fn xml_to_json_value(xml: &str) -> Value {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);
    let mut buf = Vec::new();
    let mut current_element = String::new();
    let mut json_map = HashMap::new();
    let mut stack: Vec<HashMap<String, Value>> = vec![HashMap::new()];

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                current_element =
                    String::from_utf8_lossy(e.name()).into_owned();
                let mut attributes = HashMap::new();
                for attr in e.attributes() {
                    let attr = attr.unwrap();
                    let key = String::from_utf8_lossy(&attr.key).into_owned();
                    let value =
                        String::from_utf8_lossy(&attr.value).into_owned();
                    attributes.insert(key, Value::String(value));
                }
                stack.push(attributes);
            }
            Ok(Event::End(ref e)) => {
                let element_name =
                    String::from_utf8_lossy(e.name()).into_owned();
                let top = stack.pop().unwrap();
                if let Some(mut parent) = stack.last_mut() {
                    parent.insert(element_name, Value::Object(top));
                } else {
                    json_map.insert(element_name, Value::Object(top));
                }
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape_and_decode(&reader).unwrap();
                if let Some(mut parent) = stack.last_mut() {
                    parent.insert(current_element.clone(), Value::String(text));
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!(
                "Error at position {}: {:?}",
                reader.buffer_position(),
                e
            ),
            _ => {}
        }
        buf.clear();
    }

    Value::Object(json_map)
}

#[proc_macro]
pub fn xml_to_json(input: TokenStream) -> TokenStream {
    // Parse the input token as a string literal
    let input = parse_macro_input!(input as LitStr);
    let xml_content = input.value();

    // Convert XML to JSON dynamically
    let json_value = xml_to_json_value(&xml_content);

    // Generate Rust code that returns the JSON as a serde_json::Value
    let expanded = quote! {
        #json_value
    };

    // Convert the generated code into a TokenStream
    TokenStream::from(expanded)
}
