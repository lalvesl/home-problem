use super::entity::Ticker;
use anyhow::{anyhow, Result};
use gutils::serde::Deserialize;
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use std::sync::{Arc, LazyLock};
use std::{collections::HashMap, sync::Mutex};

pub static DB_TICKERS: LazyLock<HashMap<String, Vec<Ticker>>> =
    LazyLock::new(|| {
        let tickers = Arc::new(Mutex::new(HashMap::new()));
        let runtime_tokio = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let cloned_tickers = tickers.clone();
        let handle = std::thread::spawn(move || {
            log::info!("Start get information");
            runtime_tokio.block_on(async {
                let ticks = load_tickers().await.unwrap();
                let mut buf = cloned_tickers.lock().unwrap();
                *buf = ticks;
            });
        });

        handle.join().unwrap();

        let tickers = tickers.lock().unwrap();
        tickers.clone()
    });

static HEADERS: LazyLock<HeaderMap> = LazyLock::new(|| {
    let mut header = header::HeaderMap::new();
    header.insert(
        header::USER_AGENT,
        HeaderValue::from_static("John Doe (test@example.com)"),
    );

    header.insert(
        header::ACCEPT_ENCODING,
        HeaderValue::from_static("gzip,deflate"),
    );

    header
});

async fn request(url: &str) -> Result<String> {
    let client = Client::builder().default_headers(HEADERS.clone()).build()?;

    let res = client.get(url).send().await?.text().await?;

    Ok(res)
}

#[derive(Deserialize, Debug)]
struct SecGov {
    // pub fields: Vec<String>, //not necessary field
    pub data: Vec<Vec<serde_json::Value>>,
}

async fn load_tickers() -> Result<HashMap<String, Vec<Ticker>>> {
    let json_str =
        request("https://www.sec.gov/files/company_tickers_exchange.json")
            .await?;

    let tickers: SecGov = serde_json::from_str(&json_str)?;

    let mut tickers_map: HashMap<String, Vec<Ticker>> = HashMap::new();

    for ticker in tickers.data {
        let ticker = Ticker {
            cik: ticker[0].as_u64().ok_or(anyhow!(
                "Invalid type in cik, expected number, ticker:{ticker:?}"
            ))?,
            name: ticker[1].as_str().map(|s| s.to_string()).ok_or(anyhow!(
                "Invalid type in name, expected String, ticker:{ticker:?}"
            ))?,
            ticker: ticker[2].as_str().map(|s| s.to_string()).ok_or(
                anyhow!(
                "Invalid type in ticker, expected String, ticker:{ticker:?}"
            ),
            )?,
            exchange: ticker[3].as_str().map(|s| s.to_string()),
        };

        let ticker_ref =
            tickers_map.entry(ticker.ticker.to_owned()).or_default();
        ticker_ref.push(ticker);
    }

    Ok(tickers_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn t_execute() -> Result<()> {
        let example = DB_TICKERS.get("NVDA");

        assert!(example.is_some());

        Ok(())
    }
}
