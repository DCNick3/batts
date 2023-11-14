use crate::routes::UploadPolicy;
use custom_debug::Debug;
use serde::Deserialize;
use snafu::{ResultExt, Whatever};
use std::net::SocketAddr;
use std::time::Duration;
use url::Url;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub server: Server,
    pub routes: Routes,
    pub auth: Auth,
    pub upload: Option<Upload>,
    pub storage: Storage,
}

impl Config {
    pub fn load(environment: &str) -> Result<Config, Whatever> {
        let config = config::Config::builder()
            .add_source(config::File::new("config.yaml", config::FileFormat::Yaml).required(false))
            .add_source(
                config::File::new("config.local.yaml", config::FileFormat::Yaml).required(false),
            )
            .add_source(
                config::File::new(
                    &format!("config.{}.yaml", environment),
                    config::FileFormat::Yaml,
                )
                .required(false),
            )
            .add_source(
                config::File::new(
                    &format!("config.{}.local.yaml", environment),
                    config::FileFormat::Yaml,
                )
                .required(false),
            )
            .add_source(
                config::Environment::with_prefix("config")
                    .prefix_separator("_")
                    .separator("__")
                    .list_separator(","),
            )
            .build()
            .whatever_context("Building the config file")?;

        config
            .try_deserialize()
            .whatever_context("Deserializing config structure failed")
    }
}

/// Stores the sha256 of bot token. Used to verify the login data from telegram (see https://core.telegram.org/widgets/login).
#[derive(Clone, Deserialize)]
#[serde(transparent)]
pub struct TelegramSecret(#[serde(with = "hex_serde")] pub [u8; 32]);

impl std::fmt::Debug for TelegramSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[REDACTED]")
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct Auth {
    #[serde(with = "humantime_serde")]
    pub token_duration: Duration,
    pub telegram_secret: Option<TelegramSecret>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Routes {
    pub expose_internal: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Server {
    pub endpoint: SocketAddr,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Upload {
    pub policy: UploadPolicy,
    pub s3: S3Config,
}

#[derive(Deserialize, Clone, Debug)]
pub struct S3Config {
    #[debug(format = "{}")]
    pub endpoint: Url,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Storage {
    // meilisearch is mis-used as view repository
    // and to make searches (wow)
    pub meilisearch: Meilisearch,
    // TODO: specify postgres creds when event store will be persistent
}

#[derive(Deserialize, Clone, Debug)]
pub struct Meilisearch {
    #[debug(format = "{}")]
    pub endpoint: Url,
    pub api_key: String,
}
