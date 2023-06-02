use config::Config;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigFile {
    pub bot_token: String,
    pub server_address: String,
    pub ports: Vec<u32>,
    pub location: String,
    pub admin_list: Vec<i64>,
    pub log_chat: i64,
    pub prefix: String,
}

impl fmt::Display for ConfigFile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "host: `{}`\nlocation: `{}`\nports: `{:?}`",
            self.server_address, self.location, self.ports
        )
    }
}

impl ConfigFile {
    pub fn load() -> Result<ConfigFile, Box<dyn std::error::Error>> {
        let settings = Config::builder()
            .add_source(config::File::with_name("/etc/userbot.json"))
            .build()?;

        Ok(settings.try_deserialize::<ConfigFile>()?)
    }
}
