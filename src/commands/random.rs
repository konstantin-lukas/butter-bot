use rand::Rng;
use rand_distr::{Distribution, Binomial};
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};
use crate::utils::parse::CommandArgs;

pub async fn run(args: CommandArgs) -> String {
    if let (Some(min), Some(max)) = (args.get("min"), args.get("max")) {
        return if let (Ok(min), Ok(max)) = (min.parse::<i32>(), max.parse::<i32>()) {
            if min > max {
                return String::from("Min cannot be larger than max.");
            }
            let default = String::from("uni");
            let dist = args.get("dist").unwrap_or(&default).as_str();
            let char_count = dist.chars().count();
            if char_count < 3 {
                return String::from("Invalid argument for distribution function.");
            }
            match &dist[0..=2] {
                "bin" if char_count > 3 => {
                    let p = if dist.chars().nth(3).unwrap_or('c') == 'r' {
                        0.25
                    } else if dist.chars().nth(3).unwrap_or('c') == 'l' {
                        0.75
                    } else {
                        0.5
                    };
                    let range = (max as i64 - min as i64) as u64;
                    let bin = Binomial::new(range, p).unwrap();
                    (bin.sample(&mut rand::thread_rng()) as i128).saturating_add(min as i128).to_string()
                }
                _ => {
                    let mut rng = rand::thread_rng();
                    rng.gen_range(min..=max).to_string()
                }
            }
        } else {
            String::from("Min and max have to be valid 32-bit signed integers.")
        }
    }

    String::from("Please provide all necessary arguments.")
}

pub fn register() -> CreateCommand {
    CreateCommand::new("random")
        .description("Get a random whole number between min and max.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "min",
                "The smallest possible number (inclusive)"
            ).required(true)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "max",
                "The largest possible number (inclusive)"
            ).required(true)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "dist",
                "The discrete distribution function to use. Defaults to uniform."
            )
                .required(false)
                .add_string_choice("Uniform", "uni")
                .add_string_choice("Binomial (centered)", "binc")
                .add_string_choice("Binomial (right-skewed)", "binr")
                .add_string_choice("Binomial (left-skewed)", "binl")
        )
}