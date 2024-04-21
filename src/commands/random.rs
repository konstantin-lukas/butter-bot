use rand::Rng;
use rand_distr::{Distribution, Normal};
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};
use crate::utils::parse::CommandArgs;

pub async fn run(args: CommandArgs) -> String {
    if let (Some(min), Some(max)) = (args.get("min"), args.get("max")) {
        return if let (Ok(min), Ok(max)) = (min.parse::<i32>(), max.parse::<i32>()) {
            if min > max {
                return String::from("Min cannot be larger than max.");
            }
            let mut rng = rand::thread_rng();
            match args.get("dist").unwrap_or(&String::from("uni")).as_str() {
                "nrm" => {
                    let range = max as f64 - min as f64;
                    let mean = (max as f64 + min as f64) / 2.0;
                    let standard_deviation = range / 4.5;
                    let gaussian = Normal::new(mean, standard_deviation).unwrap();
                    loop {
                        let sample = gaussian.sample(&mut rng);
                        let value = sample.round() as i32;
                        if value >= min && value <= max {
                            break value.to_string();
                        }
                    }
                }
                _ => {
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
                CommandOptionType::Integer,
                "min",
                "The smallest possible number (inclusive)"
            ).required(true)
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Integer,
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
                .add_string_choice("Normal", "nrm")
        )
}