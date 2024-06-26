mod commands;
mod crawl;
mod utils;

use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use serenity::all::{GuildId, PartialGuild};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Interaction};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::time::Duration;
use dotenvy::dotenv;
use tokio::task::JoinHandle;
use crate::commands::poll::{Poll};
use crate::utils::parse::get_command_args;

struct Handler {
    poll: Arc<Mutex<Poll>>,
    reminders: Arc<Mutex<HashMap<u64, JoinHandle<()>>>>
}

impl Handler {
    fn new() -> Self {
        Self {
            poll: Arc::new(Mutex::new(Poll::new())),
            reminders: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);


        let http = ctx.http.clone();
        let guild_id: GuildId = env::var("GUILD_ID").expect("GUILD_ID environment variable not set")
            .parse().expect("Failed to parse ADMIN_ID as u64");
        let guild = ctx.http.get_guild(guild_id).await.expect("Guild not found");

        let commands = PartialGuild::get_commands(&guild, &ctx.http).await.unwrap_or_default();

        /*if let Ok(cmds) = PartialGuild::get_commands(&guild, &ctx.http).await {
            for cmd in cmds {
                match PartialGuild::delete_command(&guild, &ctx.http, cmd.id).await {
                    Ok(_) => { println!("Deleted old command: {}",cmd.name); }
                    Err(_) => { println!("Failed to delete old command: {}",cmd.name); }
                }
            }
        };*/

        // REGISTER NEW GUILD COMMANDS
        if env::var("DEEPL_API_KEY").is_ok() && !commands.iter().any(|c| c.name == "translate") {
            match PartialGuild::create_command(&guild, &ctx.http, commands::translate::register()).await {
                Ok(_) => println!("Created new command: translate"),
                Err(e) => println!("Failed to create new command: translate - {e}")
            }
        }

        if env::var("STEAM_API_KEY").is_ok() && !commands.iter().any(|c| c.name == "common-games") {
            match PartialGuild::create_command(&guild, &ctx.http, commands::common_games::register()).await {
                Ok(_) => println!("Created new command: common-games"),
                Err(e) => println!("Failed to create new command: common-games - {e}")
            }
        }

        if !commands.iter().any(|c| c.name == "poll") {
            match PartialGuild::create_command(&guild, &ctx.http, commands::poll::register()).await {
                Ok(_) => println!("Created new command: poll"),
                Err(e) => println!("Failed to create new command: poll - {e}")
            };
        }

        if !commands.iter().any(|c| c.name == "vote") {
            match PartialGuild::create_command(&guild, &ctx.http, commands::vote::register()).await {
                Ok(_) => println!("Created new command: vote"),
                Err(e) => println!("Failed to create new command: vote - {e}")
            }
        }

        if !commands.iter().any(|c| c.name == "random") {
            match PartialGuild::create_command(&guild, &ctx.http, commands::random::register()).await {
                Ok(_) => println!("Created new command: random"),
                Err(e) => println!("Failed to create new command: random - {e}")
            }
        }

        if !commands.iter().any(|c| c.name == "remind") {
            match PartialGuild::create_command(&guild, &ctx.http, commands::remind::register()).await {
                Ok(_) => println!("Created new command: remind"),
                Err(e) => println!("Failed to create new command: remind - {e}")
            }
        }

        if env::var("DBD_CHANNEL").is_ok() {
            const INTERVAL: u64 = 60 * 60 * 24 * 1;
            tokio::spawn(async move {
                loop {
                    crawl::dbd::run(http.clone()).await;
                    tokio::time::sleep(Duration::from_secs(INTERVAL)).await;
                }
            });
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let args = get_command_args(&command.data.options());
            let content = match command.data.name.as_str() {
                "translate" if env::var("DEEPL_API_KEY").is_ok() => Some(commands::translate::run(args).await),
                "common-games" if env::var("STEAM_API_KEY").is_ok() => Some(commands::common_games::run(args).await),
                "poll" => {
                    let mut polls = self.poll.lock().await;
                    Some(commands::poll::run(args, &mut polls).await)
                },
                "vote" => {
                    let mut polls = self.poll.lock().await;
                    Some(commands::vote::run(args, &mut polls, command.user.id.get()).await)
                },
                "random" => Some(commands::random::run(args).await),
                "remind" => {
                    let mut reminders = self.reminders.lock().await;
                    Some(commands::remind::run(args, ctx.http.clone(), command.user.clone(), &mut reminders).await)
                },
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
        .event_handler(Handler::new())
        .await
        .expect("Error creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}