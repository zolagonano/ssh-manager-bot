pub mod config;

use byteorder::{ByteOrder, LittleEndian};
use chrono::{Duration, Local, NaiveDate};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use image::{load_from_memory, DynamicImage, ImageOutputFormat, Luma, LumaA, Pixel, Rgb};
use pwhash::sha512_crypt;
use qrcode::QrCode;
use rand::prelude::*;
use regex::Regex;
use std::fmt;
use std::io::{Read, Write};
use std::process::Command;
use time::{format_description::parse, macros::format_description, Date};

pub struct UserStatus {
    pub username: String,
    pub status: String,
}

impl fmt::Display for UserStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "username: `{}`\nstatus: `{}`",
            self.username, self.status
        )
    }
}

pub struct UserMax {
    pub username: String,
    pub max_logins: String,
}

impl fmt::Display for UserMax {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "username: `{}`\nmax logins: `{}`",
            self.username, self.max_logins
        )
    }
}

pub struct UserPass {
    pub username: String,
    pub password: String,
}

impl fmt::Display for UserPass {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "username: `{}`\npassword: `{}`",
            self.username, self.password
        )
    }
}

pub struct UserExp {
    pub username: String,
    pub exp_date: String,
}

impl fmt::Display for UserExp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "username: {}\nexpiry date: {}",
            self.username, self.exp_date
        )
    }
}

/// Represents the user information.
pub struct SSHUser {
    pub username: String,
    pub password: String,
    pub max_logins: String,
    pub expiry_date: String,
}

/// Displays the SSH user information in a human-readable format.
impl fmt::Display for SSHUser {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "username: `{}`\npassword: `{}`\nmax logins: `{}`\nexpiry date: `{}`",
            self.username, self.password, self.max_logins, self.expiry_date,
        )
    }
}

/// Creates a new SSH user with the provided information.
///
/// # Arguments
///
/// * `username` - The username for the new SSH user.
/// * `group` - The user group for the new SSH user.
/// * `password` - The password for the new SSH user.
/// * `exp_date` - The expiry date for the new SSH user's account.
///
/// # Returns
///
/// A `Result` containing the `SSHUser` if successful, or an error message if the user creation fails.
pub fn newuser(
    username: &str,
    group: &str,
    password: &str,
    exp_date: &str,
) -> Result<SSHUser, String> {
    let exp_date = format_exp_date(&exp_date)?;
    let password_hash = hash_password(password);
    let process_status = Command::new("useradd")
        .arg("-p")
        .arg(&password_hash)
        .arg("-s")
        .arg("/bin/rbash")
        .arg("-g")
        .arg(&group)
        .arg("-e")
        .arg(&exp_date)
        .arg(&username)
        .status();

    match process_status {
        Ok(status) => {
            if let Some(error) = unixuser_code_to_err(status.code()) {
                Err(error)
            } else {
                Ok(SSHUser {
                    username: username.to_string(),
                    password: password.to_string(),
                    max_logins: group.replace("max", "").to_string(),
                    expiry_date: exp_date.to_string(),
                })
            }
        }
        Err(_) => Err("Command useradd not found".to_string()),
    }
}

/// Automatically generates a new SSH user based on certain parameters.
///
/// # Arguments
///
/// * `prefix` - The prefix for the username.
/// * `group` - The user group for the new SSH user.
/// * `days` - The number of days until the account expiry.
///
/// # Returns
///
/// A `Result` containing the automatically generated `SSHUser` if successful, or an error message if
/// the user creation fails.
pub fn auto_newuser(prefix: &str, group: &str, days: i64) -> Result<SSHUser, String> {
    let password = gen_password();

    let users_count = get_users_core(prefix, None).len();

    let username = format!("{}{:03}",prefix, users_count + 1);
    let exp_date = add_to_time(days + 1);

    newuser(&username, group, &password, &exp_date)
}

pub fn unlock_user(username: &str) -> Result<UserStatus, String> {
    let process_status = Command::new("usermod").arg(username).arg("-U").status();

    match process_status {
        Ok(status) => {
            if let Some(error) = unixuser_code_to_err(status.code()) {
                Err(error)
            } else {
                Ok(UserStatus {
                    username: username.to_string(),
                    status: "Unlocked".to_string(),
                })
            }
        }
        Err(_) => Err("Command chage not found".to_string()),
    }
}

pub fn userdel(username: &str) -> Result<UserStatus, String> {
    let process_status = Command::new("userdel").arg(username).status();

    match process_status {
        Ok(status) => {
            if let Some(error) = unixuser_code_to_err(status.code()) {
                Err(error)
            } else {
                Ok(UserStatus {
                    username: username.to_string(),
                    status: "Deleted".to_string(),
                })
            }
        }
        Err(_) => Err("Command usermod not found".to_string()),
    }
}

pub fn change_max(username: &str, group: &str) -> Result<UserMax, String> {
    let process_status = Command::new("usermod")
        .arg(username)
        .arg("-g")
        .arg(&group)
        .status();

    match process_status {
        Ok(status) => {
            if let Some(error) = unixuser_code_to_err(status.code()) {
                Err(error)
            } else {
                Ok(UserMax {
                    username: username.to_string(),
                    max_logins: group.replace("max", ""),
                })
            }
        }
        Err(_) => Err("Command usermod not found".to_string()),
    }
}

pub fn change_pass(username: &str, password: &str) -> Result<UserPass, String> {
    let password_hash = hash_password(password);
    let process_status = Command::new("usermod")
        .arg(username)
        .arg("-p")
        .arg(&password_hash)
        .status();

    match process_status {
        Ok(status) => {
            if let Some(error) = unixuser_code_to_err(status.code()) {
                Err(error)
            } else {
                Ok(UserPass {
                    username: username.to_string(),
                    password: password.to_string(),
                })
            }
        }
        Err(_) => Err("Command usermod not found".to_string()),
    }
}

pub fn lock_user(username: &str) -> Result<UserStatus, String> {
    let process_status = Command::new("usermod").arg(username).arg("-L").status();

    match process_status {
        Ok(status) => {
            if let Some(error) = unixuser_code_to_err(status.code()) {
                Err(error)
            } else {
                Ok(UserStatus {
                    username: username.to_string(),
                    status: "Locked".to_string(),
                })
            }
        }
        Err(_) => Err("Command usermod not found".to_string()),
    }
}

pub fn change_exp(username: &str, exp_date: &str) -> Result<UserExp, String> {
    let exp_date = format_exp_date(&exp_date)?;

    let process_status = Command::new("chage")
        .arg(username)
        .arg("-E")
        .arg(&exp_date)
        .status();

    match process_status {
        Ok(status) => {
            if let Some(error) = unixuser_code_to_err(status.code()) {
                Err(error)
            } else {
                Ok(UserExp {
                    username: username.to_string(),
                    exp_date: exp_date,
                })
            }
        }
        Err(_) => Err("Command chage not found".to_string()),
    }
}

pub fn renew_user(username: &str, days: i64) -> Result<UserExp, String> {
    let exp_date = add_to_time(days + 1);

    let process_status = Command::new("chage")
        .arg(username)
        .arg("-E")
        .arg(&exp_date)
        .status();

    match process_status {
        Ok(status) => {
            if let Some(error) = unixuser_code_to_err(status.code()) {
                Err(error)
            } else {
                Ok(UserExp {
                    username: username.to_string(),
                    exp_date: exp_date,
                })
            }
        }
        Err(_) => Err("Command chage not found".to_string()),
    }
}

pub fn get_chage_exp(username: &str) -> Result<UserExp, String> {
    let process_output = Command::new("chage").arg("-l").arg(username).output();
    match process_output {
        Ok(output) => {
            if let Some(error) = unixuser_code_to_err(output.status.code()) {
                Err(error)
            } else {
                let user_info = String::from_utf8(output.stdout).unwrap();

                let re = Regex::new("Account expires\t+: (.*)\n").unwrap();

                let caps = re.captures(&user_info);

                if let Some(caps) = caps {
                    let exp_date = caps.get(1).map_or("", |m| m.as_str()).to_string();
                    if &exp_date == "never" {
                        return Ok(UserExp {
                            username: username.to_string(),
                            exp_date: "never".to_string(),
                        });
                    }

                    let inp_format = format_description!("[month repr:short] [day], [year]");

                    let out_format = format_description!("[year]-[month]-[day]");
                    match Date::parse(&exp_date, &inp_format) {
                        Ok(date) => Ok(UserExp {
                            username: username.to_string(),
                            exp_date: date.format(&out_format).unwrap(),
                        }),
                        Err(_) => Err("Invalid Expiry date".to_string()),
                    }
                } else {
                    Err("Unexpected error".to_string())
                }
            }
        }
        Err(_e) => Err("Command chage not found".to_string()),
    }
}

fn unixuser_code_to_err(code: Option<i32>) -> Option<String> {
    if let Some(code) = code {
        match code {
            0 => None,
            1 => Some("Permission denied".to_string()),
            3 => Some("Invalid shell".to_string()),
            6 => Some("Invalid user or group".to_string()),
            9 => Some("User alreade exists".to_string()),
            _ => Some("Unexpected error".to_string()),
        }
    } else {
        Some("Process terminated".to_string())
    }
}

fn add_to_time(days: i64) -> String {
    let now = Local::now().naive_local().date();
    let future_date = now + Duration::days(days);
    let formatted_date = future_date.format("%Y-%m-%d").to_string();
    format!("{}", formatted_date)
}

pub fn get_users_core(prefix: &str, usergroup: Option<&str>) -> Vec<String> {
    let iter = unsafe { users::all_users() };
    let mut users_list: Vec<String> = Vec::new();

    for user in iter {
        let username = user.name().to_string_lossy();
        let groups = user.groups();

        if username.starts_with(prefix) {
            if let Some(usergroup) = usergroup {
                if let Some(groups) = groups {
                    let group = groups.iter().find(|g| g.name() == usergroup);
                    if group.is_some() {
                        users_list.push(username.to_string());
                    }
                }
            } else {
                users_list.push(username.to_string());
            }
        }
    }

    users_list
}

pub fn gen_password() -> String {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..100000);
    format!("SSHMGMT{:05}", random_number)
}

pub fn hash_password(password: &str) -> String {
    sha512_crypt::hash_with("$6$mENJascSdtQuhrXH", password).unwrap()
}

fn format_exp_date(exp_date: &str) -> Result<String, String> {
    if let Ok(date) = NaiveDate::parse_from_str(exp_date, "%Y-%m-%d") {
        Ok(date.format("%Y-%m-%d").to_string())
    } else {
        Err("Invalid expiry date".to_string())
    }
}

fn compress_data(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

    encoder.write_all(data)?;

    encoder.finish()
}

/// Generates a Sagernet link for SSH connection based on user and server details.
///
/// # Arguments
///
/// * `server_address` - The address of the SSH server.
/// * `port` - The port number for SSH connection.
/// * `username` - The username for SSH authentication.
/// * `password` - The password for SSH authentication.
/// * `location` - The location of the server.
/// * `exp_date` - The expiry date for the SSH user's account.
///
/// # Returns
///
/// A Sagernet link encoded for SSH connection.
pub fn sagernet_link_generator(
    server_address: &str,
    port: u32,
    username: &str,
    password: &str,
    location: &str,
    exp_date: &str,
) -> String {
    let mut kryo_bytes: Vec<u8> = b"\x00\x00\x00\x00".to_vec();

    let mut server_address_bytes: Vec<u8> = server_address.as_bytes().to_vec();
    server_address_bytes.pop();
    server_address_bytes.push(server_address.chars().last().unwrap() as u8 + 128);

    kryo_bytes.extend(server_address_bytes);

    let mut port_bytes = [0u8; 2];
    LittleEndian::write_u16(&mut port_bytes, port as u16);

    kryo_bytes.extend(port_bytes);
    kryo_bytes.extend(b"\x00\x00");

    let mut username_bytes: Vec<u8> = username.as_bytes().to_vec();
    username_bytes.pop();
    username_bytes.push(username.chars().last().unwrap() as u8 + 128);

    kryo_bytes.extend(username_bytes);

    kryo_bytes.extend(b"\x01\x00\x00\x00");

    let mut password_bytes: Vec<u8> = password.as_bytes().to_vec();
    password_bytes.pop();
    password_bytes.push(password.chars().last().unwrap() as u8 + 128);

    kryo_bytes.extend(password_bytes);

    kryo_bytes.extend(b"\x81\x01\x00\x00\x00\xa1");

    let title = format!("SpeedPing({}) {} {}", username, location, exp_date);

    kryo_bytes.extend(title.as_bytes());
    kryo_bytes.extend(b"\x00\x00\x00\x00");

    let zlib_compressed = compress_data(&kryo_bytes).unwrap();

    let base64_urlsafe = base64_url::encode(&zlib_compressed);

    format!("sn://ssh?{}", base64_urlsafe)
}

/// Generates a QR code image with the provided text.
///
/// # Arguments
///
/// * `text` - The text to be encoded in the QR code.
///
/// # Returns
///
/// A vector of bytes representing the QR code image.
pub fn encode_qr_code_to_image_bytes(text: &str) -> Vec<u8> {
    let qrcode = QrCode::new(text.as_bytes());
    let qrcode_image_buffer = qrcode
        .unwrap()
        .render::<Rgb<u8>>()
        .dark_color(Rgb([123, 255, 6]))
        .light_color(Rgb([28, 32, 31]))
        .max_dimensions(550, 550)
        .build();

    let qrcode_dynamic_image = DynamicImage::ImageRgb8(qrcode_image_buffer);

    let mut image_bytes: Vec<u8> = Vec::new();

    qrcode_dynamic_image.write_to(&mut image_bytes, ImageOutputFormat::Png);

    image_bytes
}

