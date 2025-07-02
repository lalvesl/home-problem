use log::LevelFilter;

pub fn init_logger() {
    env_logger::builder()
        .filter_level(if cfg!(test) {
            // LevelFilter::Trace
            // Disable Trace, gitlab not support more 4MB of log...
            LevelFilter::Debug
        } else {
            LevelFilter::Debug
        })
        .filter_module("tokio_postgres", LevelFilter::Info)
        .filter_module("html5ever", LevelFilter::Info)
        .filter_module("selectors", LevelFilter::Info)
        .init();
}
