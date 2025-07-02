use super::service_on_start::DB_TICKERS;
use anyhow::Ok;
use gutils::anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        LazyLock,
    },
};

//If DB_TICKERS "are" static loaded memory, then load here static too
static ACTIVITY_COUTER: LazyLock<HashMap<String, AtomicU32>> =
    LazyLock::new(|| {
        DB_TICKERS
            .keys()
            .map(|k| (k.clone(), AtomicU32::new(0)))
            .collect::<HashMap<_, _>>()
    });

pub fn activity_add(key: &str) -> Result<()> {
    match ACTIVITY_COUTER.get(key) {
        Some(atomic) => {
            atomic.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
        None => Err(anyhow!("Not found key: {key}")),
    }
}

pub fn get_activities<'key_lft>() -> Vec<(&'key_lft String, u32)> {
    ACTIVITY_COUTER
        .iter()
        .map(|(k, v)| (k, v.load(Ordering::SeqCst)))
        .filter(|(_, v)| *v != 0)
        .collect()
}
