# Butter Bot

This is an open-source discord bot developed for personal use. It is not 
available as a public bot to add to your server. If you do want to add this
bot to your server, you will have to deploy it yourself.

## Features

- Regularly crawl Dead by Daylight codes and send new ones to a channel
- `/translate`: Translate a text between two languages
- `/common-games`: Get a list of games that a list of users have in common

## Environment Variables
### BOT_TOKEN
This is the token for your bot which you can get from the Discord developer portal.

### DBD_CHANNEL 
This is a channel ID the bot uses to post Dead by Daylight codes it finds on the web.

### ADMIN_ID
This is a user ID the bot messages when an error occurs.

### DEEPL_API_KEY
This is an API key the DeepL api, which the bot uses for the `/translate` command.

### GUILD_ID
This is the server ID for the server the bot is deployed on. Slash commands
are not registered globally and instead are per guild.

### STEAM_API_KEY
This is a steam api key used for the `/common-games` command.