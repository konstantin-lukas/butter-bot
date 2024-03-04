import {Client, GatewayIntentBits, TextChannel} from 'discord.js';
import 'dotenv/config';
import {listenToCommands, registerCommands} from "./commands";
import {crawlDBDCodes} from "./crawl";

(async function main() {
    const botToken = process.env.BOT_TOKEN;
    const appId = process.env.APP_ID;
    const adminId = process.env.ADMIN_ID;
    const dbdChannelID = process.env.DBD_CHANNEL;

    if (!botToken || !appId || !adminId)
        throw new Error("Missing environment variables.");

    process.title = "butter-bot";

    const client = new Client({ intents:
            [
                GatewayIntentBits.Guilds,
                GatewayIntentBits.DirectMessages
            ]
    });

    client.on('ready', async () => {
        console.log(`Logged in as ${client?.user?.tag}!`);
        if (dbdChannelID) {
            const admin = await client.users.fetch(adminId);
            const dbdChannel = await client.channels.fetch(dbdChannelID);
            await crawlDBDCodes(admin, dbdChannel as TextChannel);
            if (dbdChannel && dbdChannel.isTextBased())
                setInterval(() => crawlDBDCodes(admin, dbdChannel as TextChannel), 1000 * 60 * 60 * 24 * 3);
        }
    });

    await registerCommands(botToken, appId);
    listenToCommands(client);
    await client.login(botToken);
})();