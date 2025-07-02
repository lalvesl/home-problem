mod api;
mod utils;

use api::service::api;
use utils::env::ENV;

pub fn run() {
    let profile = &ENV.execution_profile;

    log::info!("Start process with profile: {profile:?}");

    let runtime_tokio = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let handle = std::thread::spawn(move || {
        let profile = &ENV.execution_profile;
        log::info!("Start tokio runtime with profile: {:?}", profile);
        runtime_tokio.block_on(async {
            //Impossible to handle a multi tokio runtime in a simple JoinHandle
            let _rest_apis_server_task = api().await;
        });
    });

    handle.join().unwrap();
}
