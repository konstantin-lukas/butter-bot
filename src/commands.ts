import {Client, REST, Routes, Snowflake} from "discord.js";

export async function registerCommands(token: string, appId: Snowflake) {
    const commands = [
        {
            name: 'ping',
            description: 'Replies with Pong!',
        },
    ];

    const rest = new REST({ version: '10' }).setToken(token);

    try {
        console.log('Started refreshing application (/) commands.');
        await rest.put(Routes.applicationCommands(appId), { body: commands });
        console.log('Successfully reloaded application (/) commands.');
    } catch (error) {
        console.error(error);
    }
}

export function listenToCommands(client: Client) {
    client.on('interactionCreate', async interaction => {
        if (!interaction.isChatInputCommand()) return;
        if (interaction.commandName === 'ping') {
            await interaction.reply('Pong!');
        }
    });
}
