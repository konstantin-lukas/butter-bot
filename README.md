# Butter Bot

This is an open-source discord bot developed for personal use. It is not 
available as a public bot to add to your server. If you do want to add this
bot to your server, you will have to deploy it yourself.

## Features

- Regularly crawl Dead by Daylight codes and send new ones to a channel
- `/translate`: Translate a text between two languages
- `/common-games`: Get a list of games that a list of users have in common

## Environment Variables
### BOT_TOKEN (REQUIRED)
This is the token for your bot which you can get from the Discord developer portal.

### GUILD_ID (REQUIRED)
This is the server ID for the server the bot is deployed on. Slash commands
are not registered globally and instead are per guild.

### DBD_CHANNEL (OPTIONAL)
This is a channel ID the bot uses to post Dead by Daylight codes it finds on the web.
If you don't provide this variable, the bot won't crawl codes.

### ADMIN_ID (OPTIONAL)
This is a user ID the bot messages when an error occurs. If you don't provide this
variable the bot won't message logs to an admin on error.

### DEEPL_API_KEY (OPTIONAL)
This is an API key the DeepL api, which the bot uses for the `/translate` command.
If you don't provide this variable, the command will be unavailable.

### STEAM_API_KEY (OPTIONAL)
This is a steam api key used for the `/common-games` command.
If you don't provide this variable, the command will be unavailable.