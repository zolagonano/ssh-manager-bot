# Telegram SSH User Management Bot

## Project Description
This Rust-based Telegram bot is designed to manage SSH users on a server. It provides a set of commands for administrators to interact with user accounts, such as adding new users, changing passwords, and updating expiration dates.

## Setup

### Prerequisites
- Rust programming language installed: [Rust Installation Guide](https://www.rust-lang.org/learn/get-started)

### Building the Project
1. Clone the repository:
   ```bash
   git clone https://github.com/zolagonano/ssh-manager-bot.git
   cd ssh-manager-bot
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

### Configuration
Create a configuration file named `userbot.json` with the following structure:

```json
{
  "bot_token": "YOUR_TELEGRAM_BOT_TOKEN",
  "server_address": "YOUR_SERVER_ADDRESS",
  "ports": [22, 2222],
  "location": "Server Location",
  "admin_list": [123456789],
  "log_chat": -987654321,
  "prefix": "user_prefix_"
}
```

- `bot_token`: Your Telegram bot token.
- `server_address`: Your server's address.
- `ports`: List of SSH ports.
- `location`: Location information.
- `admin_list`: List of Telegram user IDs with admin access.
- `log_chat`: Chat ID for logging.
- `prefix`: Prefix for user accounts.

### Usage
1. Run the compiled binary:
   ```bash
   ./target/release/telegram-ssh-bot
   ```

2. Interact with the bot using Telegram commands.

## Telegram Commands

- `/help`: Display available commands.
- `/getexp <username>`: Get user's expiry date.
- `/lock <username>`: Lock user.
- `/unlock <username>`: Unlock user.
- `/userdel <username>`: Delete user.
- `/changemax <username> <group>`: Change user's max logins.
- `/changepass <username> <password>`: Change user's password.
- `/changeexp <username> <exp_date>`: Change user's expiry date.
- `/renew <username> <days>`: Renew user's expiry date.
- `/useradd <username> <group> <exp_date> <password>`: Add new user manually.
- `/autoadd <group> <days>`: Add new user automatically.

## License
This project is licensed under the [MIT License](LICENSE).
