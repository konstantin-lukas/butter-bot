use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, Http, User};
use tokio::task::JoinHandle;
use crate::utils::message::send_message;
use crate::utils::parse::CommandArgs;

pub async fn run(args: CommandArgs, http: Arc<Http>, user: User, futures: &mut HashMap<u64, JoinHandle<()>>) -> String {
    if !args.contains_key("hours") && !args.contains_key("minutes") && !args.contains_key("seconds") {
        return String::from("Please give me an amount of time to wait before I remind you.")
    }

    let mut seconds: u64 = 0;
    if let Some(h) = args.get("hours") {
        if let Ok(h) = h.parse::<u64>() {
            seconds = seconds.saturating_add(h.saturating_mul(60 * 60));
        }
    }
    if let Some(m) = args.get("minutes") {
        if let Ok(m) = m.parse::<u64>() {
            seconds = seconds.saturating_add(m.saturating_mul(60));
        }
    }
    if let Some(s) = args.get("seconds") {
        if let Ok(s) = s.parse::<u64>() {
            seconds = seconds.saturating_add(s);
        }
    }

    let reply = String::from(
        format!("I will remind you in {:02}:{:02}:{:02}",
                (seconds / 60) / 60,
                (seconds / 60) % 60,
                seconds % 60)
    );

    let id = user.id.get();
    if let Some(future) = futures.remove(&id) {
        future.abort();
    }
    let timeout = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(seconds)).await;
        if let Some(m) = args.get("what") {
            send_message(http, format!("Hey, you asked me to remind you of this: {m}").as_str(), user).await;
        } else {
            send_message(http, "Hey, just messaging to because you wanted a reminder.", user).await;
        }
    });
    futures.insert(id, timeout);

    reply

}


pub fn register() -> CreateCommand {
    CreateCommand::new("remind")
        .description("Get a reminder after a certain amount of time (only one active reminder per user)")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "hours",
                "The amount of hours until you get the reminder"
            ).required(false).min_int_value(0).max_int_value(24 * 365)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "minutes",
                "The amount of minutes until you get the reminder"
            ).required(false).min_int_value(0).max_int_value(59)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
                "seconds",
                "The amount of seconds until you get the reminder"
            ).required(false).min_int_value(0).max_int_value(59)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "what",
                "A message describing of what you want to be reminded"
            ).required(false)
        )
}