use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};
use crate::commands::poll::Poll;
use crate::utils::parse::CommandArgs;

pub async fn run(mut args: CommandArgs, poll: &mut Poll, voter_id: u64) -> String {
    if poll.is_empty() {
        return String::from("There is currently no ongoing poll 😕");
    }

    if let Some(choice) = args.remove("choice") {
        if let Some(choice) = poll.options.get_mut(&choice) {
            if poll.voters.contains(&voter_id) {
                String::from("You already voted 😕")
            } else {
                poll.voters.insert(voter_id);
                *choice += 1;
                String::from("Vote registered 🗳️")
            }
        } else {
            String::from(format!("There is no choice with the label \"{}\"", choice))
        }
    } else {
        String::from("An error occurred 💀")
    }
}


pub fn register() -> CreateCommand {
    CreateCommand::new("vote")
        .description("Vote in an ongoing poll")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "choice",
                "The choice you want to vote for"
            ).required(true)
        )
}