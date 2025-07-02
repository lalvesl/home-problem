use anyhow::{anyhow, Result};
use base64::{prelude::BASE64_STANDARD as B64, Engine};

pub fn enc<T>(input: T) -> String
where
    T: AsRef<[u8]>,
{
    B64.encode::<T>(input)
}

pub fn dec(input: &str) -> Result<String> {
    match B64.decode(input) {
        Ok(raw_data) => match String::from_utf8(raw_data) {
            Ok(out_str) => Ok(out_str),
            Err(e) => Err(anyhow!("Error on decode string, error:'{e:?}'")),
        },
        Err(e) => Err(anyhow!("Error on decode base64 string, error:'{e:?}'")),
    }
}
