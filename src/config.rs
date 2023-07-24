use config::Config;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Struct representing the configuration file for the userbot.
#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigFile {
    /// Token for the userbot's Telegram bot.
    pub bot_token: String,
    /// Address of the server where the userbot is hosted.
    pub server_address: String,
    /// List of ports used by the userbot.
    pub ports: Vec<u32>,
    /// Location information of the userbot.
    pub location: String,
    /// List of user IDs designated as administrators.
    pub admin_list: Vec<i64>,
    /// ID of the chat used for logging.
    pub log_chat: i64,
    /// Prefix used for userbot commands.
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
    /// Loads the configuration from the specified file path and returns a `ConfigFile` instance.
    pub fn load() -> Result<ConfigFile, Box<dyn std::error::Error>> {
        let settings = Config::builder()
            .add_source(config::File::with_name("/etc/userbot.json"))
            .build()?;

        Ok(settings.try_deserialize::<ConfigFile>()?)
    }
}
