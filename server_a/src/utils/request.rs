use crate::utils::env::ENV;
use anyhow::Result;
use base64::{prelude::BASE64_STANDARD as B64, Engine};
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use std::sync::LazyLock;

static AUTH: LazyLock<&str> = LazyLock::new(|| {
    format!(
        "Basic {}",
        B64.encode(format!("{}:{}", ENV.sap_soap_user, ENV.sap_soap_pass))
    )
    .leak()
});

static HEADERS: LazyLock<HeaderMap> = LazyLock::new(|| {
    let mut header = header::HeaderMap::new();
    header.insert(
        header::AUTHORIZATION,
        HeaderValue::from_static(AUTH.clone()),
    );
    header.insert(
        header::ACCEPT_ENCODING,
        HeaderValue::from_static("gzip,deflate"),
    );

    header.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/xml;charset=UTF-8"),
    );

    header.insert(
        "SOAPAction",
        HeaderValue::from_static("\"http://sap.com/xi/WebService/soap1.1\""),
    );

    header.insert(header::CONNECTION, HeaderValue::from_static("keep-alive"));

    header.insert(
        header::USER_AGENT,
        HeaderValue::from_static("Apache-HttpClient/4.5.5 (Java/17.0.12)"),
    );
    header
});

pub async fn request(url: &str, body: &str) -> Result<String> {
    let client = Client::builder().default_headers(HEADERS.clone()).build()?;

    let res = client
        .post(url)
        .body(body.to_string())
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::sap::itf_o_s_material::repository::SAP_SOAP_URL as SAP_SOAP_URL_FROM_MATERIAL;

    #[ignore]
    #[tokio::test]
    async fn t_execute() -> Result<()> {
        let body = include_str!("../itf_o_s_material/material.xml");
        request(SAP_SOAP_URL_FROM_MATERIAL, body).await?;

        Ok(())
    }
}
