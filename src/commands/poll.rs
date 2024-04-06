use std::collections::HashMap;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};
use crate::utils::parse::CommandArgs;

pub type Polls = HashMap<String, Vec<String>>;
pub async fn run(_args: CommandArgs, _polls: &mut Polls) -> String {


    String::from("OK")
}

pub fn register() -> CreateCommand {
    CreateCommand::new("poll")
        .description("Start a new poll or get results if poll already exists")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "name",
                "The name of the poll"
            ).required(true)
        )
}