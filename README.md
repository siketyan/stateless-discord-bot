# 🧲 Stateless Discord Bot
![Rust](https://github.com/siketyan/stateless-discord-bot/workflows/Rust/badge.svg)
![Rust](https://github.com/siketyan/stateless-discord-bot/workflows/Wrangler/badge.svg)

An example of stateless Discord Bot using Slash Commands feature and Cloudflare Workers.

## ✨ Features
- 🦀 Using Rust (with WebAssembly)!
- 🔌 You can create a **stateless** Discord Bot
- 🛠 Of course it is serverless
- 💰 Free for 100,000 requests with Cloudflare Workers

## 📦 Installation
1. Clone this repository.
1. Copy `wrangler.example.toml`, then rename it to `wranger.toml` .
1. Edit the file, filling `account_id` and `vars.PUBLIC_KEY` .
1. Deploy using `wrangler publish` .
1. Register your endpoint at Discord Developer Portal.
1. Create an application command. For example:
   ```console
   $ curl \
     -H "Authorization: Bot ${DISCORD_BOT_TOKEN}" \
     -H "Content-Type: application/json" \
     -d '{"name":"hello","description":"The bot will say \"Hello, world!\"."}' \
     "https://discord.com/api/v8/applications/${APPLICATION_ID}/commands"
   ```
1. Done!
