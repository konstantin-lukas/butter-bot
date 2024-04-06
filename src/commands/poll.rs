use std::collections::{HashMap, HashSet};
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};
use crate::utils::parse::CommandArgs;

pub struct Poll {
    pub voters: HashSet<u64>,
    pub options: PollOptions
}

impl Poll {
    pub fn new() -> Self {
        Self {
            voters: HashSet::new(),
            options: HashMap::new()
        }
    }

    pub fn clear(&mut self) {
        self.voters.clear();
        self.options.clear();
    }

    pub fn is_empty(&self) -> bool {
        return self.options.is_empty();
    }
}
pub type PollOptions = HashMap<String, u64>;
pub async fn run(args: CommandArgs, poll: &mut Poll) -> String {

    let user_options = args.iter().map(|(_,x)| x).collect::<Vec<_>>();

    if user_options.len() != user_options.iter().collect::<HashSet<_>>().len() {
        return String::from("You can't provide multiple options with the same name");
    }

    let mut answer;
    if poll.is_empty() {
        for user_option in user_options {
            poll.options.insert(user_option.clone(), 0);
        }
        if poll.is_empty() {
            answer = String::from("No poll was created because no choices were provided 😔");
        } else {
            answer = String::from("Created poll 🗳️\nThese are the choices: ");
            let mut options_iter = poll.options.iter();
            if let Some((first_option, _)) = options_iter.next() {
                answer.push_str(format!("`{}`", first_option).as_str());
                for (option, _) in options_iter {
                    answer.push_str(format!(", `{}`", option).as_str());
                }
            }
        }
    } else {
        answer = format!("Poll results 📊\n{}", poll
            .options
            .iter()
            .map(
                |(name, vote_count)| {
                    let word = if *vote_count == 1 { "vote" } else { "votes" };
                    return format!("{}: {} {}\n", &name, vote_count, word);
                }
            ).collect::<String>());
        poll.clear();
    }
    answer
}

pub fn register() -> CreateCommand {
    CreateCommand::new("poll")
        .description("Start a new poll or get results of ongoing poll.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "choice1",
                "Choice 1"
            ).required(false)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "choice2",
                "Choice 2"
            ).required(false)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "choice3",
                "Choice 3"
            ).required(false)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "choice4",
                "Choice 4"
            ).required(false)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "choice5",
                "Choice 5"
            ).required(false)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "choice6",
                "Choice 6"
            ).required(false)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "choice7",
                "Choice 7"
            ).required(false)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "choice8",
                "Choice 8"
            ).required(false)
        )
}