use lazy_static::lazy_static;
use lib::config;
use teloxide::types::{CallbackQuery, InputFile, ParseMode};
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide_core::types::{InlineKeyboardButton, InlineKeyboardMarkup};

lazy_static! {
    static ref CONFIG: config::ConfigFile =
        config::ConfigFile::load().unwrap_or_else(|_| panic!("Couldn't load config file!"));
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::new(&CONFIG.bot_token);
    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "get user's expiry date")]
    GetExp(String),
    #[command(description = "lock user")]
    Lock(String),
    #[command(description = "unlock user")]
    Unlock(String),
    #[command(description = "delete user")]
    UserDel(String),
    #[command(description = "change user's max logins", parse_with = "split")]
    ChangeMax { username: String, group: String },
    #[command(description = "change user's password", parse_with = "split")]
    ChangePass { username: String, password: String },
    #[command(description = "change user's expiry date", parse_with = "split")]
    ChangeExp { username: String, exp_date: String },
    #[command(description = "renew user's expiry date", parse_with = "split")]
    Renew { username: String, days: i64 },
    #[command(description = "add new user manually", parse_with = "split")]
    UserAdd {
        username: String,
        group: String,
        exp_date: String,
        password: String,
    },
    #[command(description = "add new user automatically", parse_with = "split")]
    AutoAdd { group: String, days: i64 },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::GetExp(username) => {
            if !CONFIG.admin_list.contains(&msg.chat.id.0) {
                return Ok(());
            }

            match lib::get_chage_exp(&username) {
                Ok(user_exp) => {
                    bot.send_message(msg.chat.id, format!("{user_exp}"))
                        .parse_mode(ParseMode::Markdown)
                        .await?;

                    bot.forward_message(ChatId(CONFIG.log_chat), msg.chat.id, msg.id)
                        .await?
                }
                Err(err) => bot.send_message(msg.chat.id, err).await?,
            }
        }
        Command::Lock(username) => {
            if !CONFIG.admin_list.contains(&msg.chat.id.0) {
                return Ok(());
            }

            match lib::lock_user(&username) {
                Ok(user_status) => {
                    bot.send_message(msg.chat.id, format!("{user_status}"))
                        .parse_mode(ParseMode::Markdown)
                        .await?;

                    bot.forward_message(ChatId(CONFIG.log_chat), msg.chat.id, msg.id)
                        .await?
                }
                Err(err) => bot.send_message(msg.chat.id, err).await?,
            }
        }
        Command::Unlock(username) => {
            if !CONFIG.admin_list.contains(&msg.chat.id.0) {
                return Ok(());
            }

            match lib::unlock_user(&username) {
                Ok(user_status) => {
                    bot.send_message(msg.chat.id, format!("{user_status}"))
                        .parse_mode(ParseMode::Markdown)
                        .await?;

                    bot.forward_message(ChatId(CONFIG.log_chat), msg.chat.id, msg.id)
                        .await?
                }
                Err(err) => bot.send_message(msg.chat.id, err).await?,
            }
        }
        Command::UserDel(username) => {
            if !CONFIG.admin_list.contains(&msg.chat.id.0) {
                return Ok(());
            }

            match lib::userdel(&username) {
                Ok(user_status) => {
                    bot.send_message(msg.chat.id, format!("{user_status}"))
                        .parse_mode(ParseMode::Markdown)
                        .await?;

                    bot.forward_message(ChatId(CONFIG.log_chat), msg.chat.id, msg.id)
                        .await?
                }
                Err(err) => bot.send_message(msg.chat.id, err).await?,
            }
        }
        Command::ChangeMax { username, group } => {
            if !CONFIG.admin_list.contains(&msg.chat.id.0) {
                return Ok(());
            }

            match lib::change_max(&username, &group) {
                Ok(user_max) => {
                    bot.send_message(msg.chat.id, format!("{user_max}"))
                        .parse_mode(ParseMode::Markdown)
                        .await?;

                    bot.forward_message(ChatId(CONFIG.log_chat), msg.chat.id, msg.id)
                        .await?
                }
                Err(err) => bot.send_message(msg.chat.id, err).await?,
            }
        }
        Command::ChangePass { username, password } => {
            if !CONFIG.admin_list.contains(&msg.chat.id.0) {
                return Ok(());
            }

            match lib::change_pass(&username, &password) {
                Ok(user_pass) => {
                    bot.send_message(msg.chat.id, format!("{user_pass}"))
                        .parse_mode(ParseMode::Markdown)
                        .await?;

                    bot.forward_message(ChatId(CONFIG.log_chat), msg.chat.id, msg.id)
                        .await?
                }
                Err(err) => bot.send_message(msg.chat.id, err).await?,
            }
        }
        Command::ChangeExp { username, exp_date } => {
            if !CONFIG.admin_list.contains(&msg.chat.id.0) {
                return Ok(());
            }

            match lib::change_exp(&username, &exp_date) {
                Ok(user_exp) => {
                    bot.send_message(msg.chat.id, format!("{user_exp}"))
                        .parse_mode(ParseMode::Markdown)
                        .await?;

                    bot.forward_message(ChatId(CONFIG.log_chat), msg.chat.id, msg.id)
                        .await?
                }
                Err(err) => bot.send_message(msg.chat.id, err).await?,
            }
        }
        Command::Renew { username, days } => {
            if !CONFIG.admin_list.contains(&msg.chat.id.0) {
                return Ok(());
            }

            match lib::renew_user(&username, days) {
                Ok(user_exp) => {
                    bot.send_message(msg.chat.id, format!("{user_exp}"))
                        .parse_mode(ParseMode::Markdown)
                        .await?;

                    bot.forward_message(ChatId(CONFIG.log_chat), msg.chat.id, msg.id)
                        .await?
                }
                Err(err) => bot.send_message(msg.chat.id, err).await?,
            }
        }
        Command::UserAdd {
            username,
            group,
            exp_date,
            password,
        } => {
            if !CONFIG.admin_list.contains(&msg.chat.id.0) {
                return Ok(());
            }

            let config_file: config::ConfigFile = CONFIG.clone();
            match lib::newuser(&username, &group, &password, &exp_date) {
                Ok(sshuser) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("**user info:**\n{sshuser}\n\n**server info:**\n{config_file}"),
                    )
                    .parse_mode(ParseMode::Markdown)
                    .await?;

                    let sagernet_link = lib::sagernet_link_generator(
                        &config_file.server_address,
                        config_file.ports[0],
                        &sshuser.username,
                        &sshuser.password,
                        &config_file.location,
                        &sshuser.expiry_date,
                    );

                    let qr_bytes = lib::encode_qr_code_to_image_bytes(&sagernet_link);
                    let input_file = InputFile::memory(qr_bytes);

                    bot.send_photo(msg.chat.id, input_file)
                        .caption(format!(
                            "**{}** {}\n`{sagernet_link}`",
                            &sshuser.username, &sshuser.expiry_date
                        ))
                        .parse_mode(ParseMode::Markdown)
                        .await?;

                    bot.forward_message(ChatId(CONFIG.log_chat), msg.chat.id, msg.id)
                        .await?
                }
                Err(err) => bot.send_message(msg.chat.id, err).await?,
            }
        }
        Command::AutoAdd { group, days } => {
            if !CONFIG.admin_list.contains(&msg.chat.id.0) {
                return Ok(());
            }

            let config_file: config::ConfigFile = CONFIG.clone();
            match lib::auto_newuser(&config_file.prefix, &group, days) {
                Ok(sshuser) => {
                    bot.send_message(
                        msg.chat.id,
                        format!("**user info:**\n{sshuser}\n\n**server info:**\n{config_file}"),
                    )
                    .parse_mode(ParseMode::Markdown)
                    .await?;

                    let sagernet_link = lib::sagernet_link_generator(
                        &config_file.server_address,
                        config_file.ports[0],
                        &sshuser.username,
                        &sshuser.password,
                        &config_file.location,
                        &sshuser.expiry_date,
                    );

                    let qr_bytes = lib::encode_qr_code_to_image_bytes(&sagernet_link);
                    let input_file = InputFile::memory(qr_bytes);

                    bot.send_photo(msg.chat.id, input_file)
                        .caption(format!(
                            "**{}** {}\n`{sagernet_link}`",
                            &sshuser.username, &sshuser.expiry_date
                        ))
                        .parse_mode(ParseMode::Markdown)
                        .await?;

                    bot.forward_message(ChatId(CONFIG.log_chat), msg.chat.id, msg.id)
                        .await?
                }
                Err(err) => bot.send_message(msg.chat.id, err).await?,
            }
        }
    };

    Ok(())
}

