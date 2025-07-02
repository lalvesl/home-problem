#[allow(non_snake_case)]
pub mod PATHS {
    use std::{env, sync::LazyLock};

    pub static CURRENT_DIR: LazyLock<&'static str> = LazyLock::new(|| {
        #[cfg(debug_assertions)]
        {
            let mut current_dir = env::current_dir().unwrap();

            while current_dir.exists() {
                if current_dir.join("target").is_dir() {
                    break;
                }

                if !current_dir.pop() {
                    break;
                }
            }

            println!("Current path is: \"{:?}\"", current_dir);

            current_dir
                .canonicalize()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                .leak()
        }

        #[cfg(not(debug_assertions))]
        env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .leak()
    });
}
