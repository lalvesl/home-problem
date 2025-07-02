use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};
use std::fmt;
use utoipa::{IntoParams, ToSchema};

#[derive(
    IntoParams, Serialize, Debug, PartialEq, Eq, ToSchema, Default, Clone,
)]
pub struct Ticker {
    pub cik: u64,
    pub name: String,
    pub ticker: String,
    pub exchange: Option<String>,
}

impl<'de> Deserialize<'de> for Ticker {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct TickerVisitor;

        impl<'de> Visitor<'de> for TickerVisitor {
            type Value = Ticker;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "A list of elements like cik, name, ticker, exchange",
                )
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let ticker = Ticker {
                    cik: seq.next_element()?.ok_or_else(|| {
                        de::Error::invalid_value(de::Unexpected::Seq, &self)
                    })?,
                    name: seq.next_element()?.ok_or_else(|| {
                        de::Error::invalid_value(de::Unexpected::Seq, &self)
                    })?,
                    ticker: seq.next_element()?.ok_or_else(|| {
                        de::Error::invalid_value(de::Unexpected::Seq, &self)
                    })?,
                    exchange: seq.next_element()?.ok_or_else(|| {
                        de::Error::invalid_value(de::Unexpected::Seq, &self)
                    })?,
                };

                Ok(ticker)
            }
        }

        deserializer.deserialize_seq(TickerVisitor)
    }
}
