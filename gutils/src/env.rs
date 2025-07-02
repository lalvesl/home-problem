use super::consts::PATHS::CURRENT_DIR;
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Profile {
    DEV,
    QA,
    PRD,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Env {
    pub execution_profile: Profile,
    pub server_a_port: u16,
}

impl Env {
    pub fn load() -> Env {
        let make_path = |path: &str| -> Result<String> {
            Ok(Path::new(&CURRENT_DIR.to_string())
                .join(path)
                .canonicalize()?
                .to_str()
                .unwrap()
                .to_string())
        };

        for path in ["../.env", ".env"] {
            if let Ok(path) = make_path(path) {
                if fs::read(&path).is_ok() {
                    dotenv::from_path(&path)
                        .expect("Invalid format of .env file")
                }
            }
        }

        let profile =
            std::env::var("EXECUTION_PROFILE").unwrap_or("DEV".to_string());

        let execution_profile = match profile.as_str() {
            "DEV" => Profile::DEV,
            "PRD" => Profile::PRD,
            "QA" => Profile::QA,
            _ => Profile::QA,
        };

        Env {
            execution_profile,
            server_a_port: std::env::var("SERVER_A_PORT")
                .map(|s| s.parse().expect("Incorrect number on SERVER_PORT"))
                .expect("Incorrect SERVER_PORT Env"),
        }
    }
}

pub static ENV: LazyLock<Env> = LazyLock::new(|| {
    let env = Env::load();
    super::logger::init_logger();
    env
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_env() {
        let _env = Env::load();
    }
}
