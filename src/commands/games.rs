use std::env;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue};
use std::collections::HashMap;
use reqwest::{Client};
use serenity::json::Value;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Game {
    appid: u32,
    name: String,
}

#[derive(Debug, Deserialize)]
struct OwnedGamesResponse {
    response: OwnedGames,
}

#[derive(Debug, Deserialize)]
struct OwnedGames {
    games: Vec<Game>,
}

async fn get_steam_user_ids(usernames: &Vec<&&str>, api_key: &str) -> HashMap<String, String> {
    let client = Client::new();
    let mut user_ids = HashMap::new();

    let tasks = usernames.into_iter().map(|username| {
        let client = client.clone();
        async move {
            let url = format!(
                "https://api.steampowered.com/ISteamUser/ResolveVanityURL/v1/?key={api_key}&vanityurl={username}"
            );

            match client.get(&url).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        let user_info: Value = response.json().await.unwrap_or_default();
                        let user_id = user_info["response"]["steamid"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                        (username.to_string(), user_id)
                    } else {
                        (username.to_string(), "Error: Unable to fetch user ID".to_string())
                    }
                }
                Err(_) => {
                    (username.to_string(), "Error: Request failed".to_string())
                }
            }
        }
    });

    let results = serenity::futures::future::join_all(tasks).await;
    for (username, user_id) in results {
        user_ids.insert(username, user_id);
    }

    user_ids
}

async fn get_owned_games(user_id: &str, api_key: &str) -> Vec<Game> {
    let client = Client::new();
    let url = format!(
        "https://api.steampowered.com/IPlayerService/GetOwnedGames/v1/?key={api_key}&steamid={user_id}&include_appinfo=1&include_played_free_games=1"
    );

    let response = client.get(&url).send().await;

    if let Ok(r) = response {
        if r.status().is_success() {
            if let Ok(owned) = r.json::<OwnedGamesResponse>().await {
                return owned.response.games;
            }
        }
    }
    Vec::new()
}

fn is_numeric(input: &str) -> bool {
    for c in input.chars() {
        if !c.is_digit(10) {
            return false;
        }
    }
    true
}

pub async fn run(options: &[ResolvedOption<'_>]) -> String {
    let api_key = match env::var("STEAM_API_KEY") {
        Ok(x) => x,
        Err(_) => { return String::from("Sorry, I don't currently have a Steam API key :c"); }
    };



    let mut users = Vec::new();
    for x in options {
        if let ResolvedOption { value: ResolvedValue::String(user), .. } = x {
            users.push(user);
        }
    }
    let mut ids = get_steam_user_ids(&users, &api_key).await;

    for (name, id) in ids.clone() {
        let copy = String::from(name);
        let copy2 = copy.clone();
        if id == "" {
            if is_numeric(copy2.as_str()) {
                ids.insert(copy, copy2);
            } else {
                return String::from(format!("I'm sorry, but {copy} doesn't have a custom url associated \
                with their steam account. You can still use this feature by going to {copy}'s account \
                and copying the ID number from the end of their profile url."))
            }
        }
    }


    let tasks = ids.into_iter().map(|(_, id)| {
        let api_key = api_key.clone();
        async move {
            get_owned_games(&id, &api_key).await
        }
    });

    let games = serenity::futures::future::join_all(tasks).await;


    let mut appid_count: HashMap<(u32, &str), usize> = HashMap::new();
    for games_inner in &games {
        for game in games_inner {
            let count = appid_count.entry((game.appid,game.name.as_str())).or_default();
            *count += 1;
        }
    }

    let mut reply = String::from("Here's a list of games you have in common:\n");
    let mut found_matches = false;
    for ((_, name), count ) in appid_count {
        if count == users.len() {
            reply.push_str(format!("- `{name}`\n").as_str());
            found_matches = true;
        }
    }
    if !found_matches {
        String::from("Seems like you have no games in common.")
    } else {
        reply
    }
}

pub fn register() -> CreateCommand {

    CreateCommand::new("common-games")
        .description("Get a list of games steam users have in common")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "user1",
                "Username as it appears in the account url"
            ).required(true)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "user2",
                "Username as it appears in the account url"
            ).required(true)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "user3",
                "Username as it appears in the account url"
            ).required(false)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "user4",
                "Username as it appears in the account url"
            ).required(false)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "user5",
                "Username as it appears in the account url"
            ).required(false)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "user6",
                "Username as it appears in the account url"
            ).required(false)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "user7",
                "Username as it appears in the account url"
            ).required(false)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "user8",
                "Username as it appears in the account url"
            ).required(false)
        )
}