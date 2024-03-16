mod commands;
mod crawl;
mod utils;

use std::env;
use serenity::all::{ChannelId, GuildId, PartialGuild, UserId};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use crate::utils::message::MessageHandler;
use std::time::Duration;
use dotenvy::dotenv;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let admin_id: u64 = env::var("ADMIN_ID").expect("ADMIN_ID environment variable not set")
            .parse().expect("Failed to parse ADMIN_ID as u64");
        let dbd_channel_id: u64 = env::var("DBD_CHANNEL").expect("DBD_CHANNEL environment variable not set")
            .parse().expect("Failed to parse DBD_CHANNEL as u64");

        let http = ctx.http.clone();
        let admin = http.get_user(UserId::new(admin_id)).await.expect("Admin user not found");
        let dbd_channel = ChannelId::new(dbd_channel_id);
        let message_handler = MessageHandler::new(http, admin, dbd_channel);
        let guild_id: GuildId = env::var("GUILD_ID").expect("GUILD_ID environment variable not set")
            .parse().expect("Failed to parse ADMIN_ID as u64");
        let guild = ctx.http.get_guild(guild_id).await.expect("Guild not found");

        // DELETE ALL GUILD COMMANDS
        if let Ok(cmds) = PartialGuild::get_commands(&guild, &ctx.http).await {
            for cmd in cmds {
                match PartialGuild::delete_command(&guild, &ctx.http, cmd.id).await {
                    Ok(_) => { println!("Deleted command: {}",cmd.name); }
                    Err(_) => { println!("Failed to delete command: {}",cmd.name); }
                }
            }
        };

        PartialGuild::create_command(&guild, &ctx.http, commands::translate::register())
            .await.expect("Failed to create translate command!");
        PartialGuild::create_command(&guild, &ctx.http, commands::games::register())
            .await.expect("Failed to create common games command!");

        const INTERVAL: u64 = 60 * 60 * 24 * 2;
        tokio::spawn(async move {
            loop {
                crawl::dbd::run(&message_handler).await;
                tokio::time::sleep(Duration::from_secs(INTERVAL)).await;
            }
        });

    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {

            let content = match command.data.name.as_str() {
                "translate" => Some(commands::translate::run(&command.data.options()).await),
                "common-games" => Some(commands::games::run(&command.data.options()).await),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {

    dotenv().expect(".env file not found");
    let token = env::var("BOT_TOKEN").expect("BOT_TOKEN environment variable not set");
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}