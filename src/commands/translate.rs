use std::env;
use deepl::{DeepLApi, Lang};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use serenity::all::{CommandOptionType, CreateCommandOption, ResolvedValue};

struct Language<'a> {
    code: &'a str,
    name: &'a str
}

fn str_to_lang(input: &str, source: bool) -> Lang {
    match input {
        "BG" => Lang::BG,
        "CS" => Lang::CS,
        "DA" => Lang::DA,
        "DE" => Lang::DE,
        "EL" => Lang::EL,
        "ES" => Lang::ES,
        "FI" => Lang::FI,
        "FR" => Lang::FR,
        "HU" => Lang::HU,
        "ID" => Lang::ID,
        "IT" => Lang::IT,
        "JA" => Lang::JA,
        "KO" => Lang::KO,
        "LT" => Lang::LT,
        "NB" => Lang::NB,
        "NL" => Lang::NL,
        "PL" => Lang::PL,
        "PT" => if source {Lang::PT} else {Lang::PT_BR} ,
        "RU" => Lang::RU,
        "SK" => Lang::SK,
        "SV" => Lang::SV,
        "TR" => Lang::TR,
        "UK" => Lang::TR,
        "ZH" => Lang::TR,
        _ => if source {Lang::EN} else {Lang::EN_US}
    }
}

const LANGUAGES: [Language<'static>; 25] = [
    Language{ code: "BG", name: "Bulgarian" },
    Language{ code: "CS", name: "Czech" },
    Language{ code: "DA", name: "Danish" },
    Language{ code: "DE", name: "German" },
    Language{ code: "EL", name: "Greek" },
    Language{ code: "EN", name: "English" },
    Language{ code: "ES", name: "Spanish" },
    Language{ code: "FI", name: "Finnish" },
    Language{ code: "FR", name: "French" },
    Language{ code: "HU", name: "Hungarian" },
    Language{ code: "ID", name: "Hungarian" },
    Language{ code: "IT", name: "Italian" },
    Language{ code: "JA", name: "Japanese" },
    Language{ code: "KO", name: "Korean" },
    Language{ code: "LT", name: "Lithuanian" },
    Language{ code: "NB", name: "Norwegian" },
    Language{ code: "NL", name: "Dutch" },
    Language{ code: "PL", name: "Polish" },
    Language{ code: "PT", name: "Portuguese" },
    Language{ code: "RU", name: "Russian" },
    Language{ code: "SK", name: "Slovak" },
    Language{ code: "SV", name: "Swedish" },
    Language{ code: "TR", name: "Turkish" },
    Language{ code: "UK", name: "Ukrainian" },
    Language{ code: "ZH", name: "Chinese" }
];

pub async fn run(options: &[ResolvedOption<'_>]) -> String {
    let api_key = match env::var("DEEPL_API_KEY") {
        Ok(i) => i,
        Err(_) => {
            return String::from(
                "Sorry, I don't currently have a key for the translation API. \
                You can blame my maker for that."
            )
        }
    };

    let mut from = "";
    let mut to= "";
    let mut text= "";
    for o in options.to_vec() {
        if o.name == "text" {
            text = match o {
                ResolvedOption { value: ResolvedValue::String(str), .. } => {
                    str
                },
                _ => return String::from("Please provide the 'text' option so I know what to translate.")
            };
        }
        if o.name == "from" {
            from = match o {
                ResolvedOption { value: ResolvedValue::String(str), .. } => {
                    str
                },
                _ => ""
            };
        }
        if o.name == "to" {
            to = match o {
                ResolvedOption { value: ResolvedValue::String(str), .. } => {
                    str
                },
                _ => "EN"
            }
        }
    }

    let api = DeepLApi::with(&api_key).new();
    let request = if from != "" {
        api.translate_text(
            text,
            str_to_lang(to, false)
        ).source_lang(str_to_lang(from, true)).await
    } else {
        api.translate_text(
            text,
            str_to_lang(to, false)
        ).await
    };

    let translated = match request {
        Ok(i) => i,
        Err(_) => {
            return String::from(
                "Sorry, I couldn't translate your request. This might be because I have reached \
                the limit of my API key this month."
            )
        }
    };

    if translated.translations.len() > 0 {
        String::from(&translated.translations[0].text)
    } else {
        String::from("Cosmic rays prevented me from translating your request.")
    }
}

pub fn register() -> CreateCommand {
    let from = CreateCommandOption::new(
        CommandOptionType::String,
        "from",
        "The source language - If omitted language will be detected automatically."
    ).required(false)
        .add_string_choice(LANGUAGES[0].name, LANGUAGES[0].code)
        .add_string_choice(LANGUAGES[1].name, LANGUAGES[1].code)
        .add_string_choice(LANGUAGES[2].name, LANGUAGES[2].code)
        .add_string_choice(LANGUAGES[3].name, LANGUAGES[3].code)
        .add_string_choice(LANGUAGES[4].name, LANGUAGES[4].code)
        .add_string_choice(LANGUAGES[5].name, LANGUAGES[5].code)
        .add_string_choice(LANGUAGES[6].name, LANGUAGES[6].code)
        .add_string_choice(LANGUAGES[7].name, LANGUAGES[7].code)
        .add_string_choice(LANGUAGES[8].name, LANGUAGES[8].code)
        .add_string_choice(LANGUAGES[9].name, LANGUAGES[9].code)
        .add_string_choice(LANGUAGES[10].name, LANGUAGES[10].code)
        .add_string_choice(LANGUAGES[11].name, LANGUAGES[11].code)
        .add_string_choice(LANGUAGES[12].name, LANGUAGES[12].code)
        .add_string_choice(LANGUAGES[13].name, LANGUAGES[13].code)
        .add_string_choice(LANGUAGES[14].name, LANGUAGES[14].code)
        .add_string_choice(LANGUAGES[15].name, LANGUAGES[15].code)
        .add_string_choice(LANGUAGES[16].name, LANGUAGES[16].code)
        .add_string_choice(LANGUAGES[17].name, LANGUAGES[17].code)
        .add_string_choice(LANGUAGES[18].name, LANGUAGES[18].code)
        .add_string_choice(LANGUAGES[19].name, LANGUAGES[19].code)
        .add_string_choice(LANGUAGES[20].name, LANGUAGES[20].code)
        .add_string_choice(LANGUAGES[21].name, LANGUAGES[21].code)
        .add_string_choice(LANGUAGES[22].name, LANGUAGES[22].code)
        .add_string_choice(LANGUAGES[23].name, LANGUAGES[23].code)
        .add_string_choice(LANGUAGES[24].name, LANGUAGES[24].code);

    let to = CreateCommandOption::new(
        CommandOptionType::String,
        "to",
        "The target language - If omitted will translate to English."
    ).required(false)
        .add_string_choice(LANGUAGES[0].name, LANGUAGES[0].code)
        .add_string_choice(LANGUAGES[1].name, LANGUAGES[1].code)
        .add_string_choice(LANGUAGES[2].name, LANGUAGES[2].code)
        .add_string_choice(LANGUAGES[3].name, LANGUAGES[3].code)
        .add_string_choice(LANGUAGES[4].name, LANGUAGES[4].code)
        .add_string_choice(LANGUAGES[5].name, LANGUAGES[5].code)
        .add_string_choice(LANGUAGES[6].name, LANGUAGES[6].code)
        .add_string_choice(LANGUAGES[7].name, LANGUAGES[7].code)
        .add_string_choice(LANGUAGES[8].name, LANGUAGES[8].code)
        .add_string_choice(LANGUAGES[9].name, LANGUAGES[9].code)
        .add_string_choice(LANGUAGES[10].name, LANGUAGES[10].code)
        .add_string_choice(LANGUAGES[11].name, LANGUAGES[11].code)
        .add_string_choice(LANGUAGES[12].name, LANGUAGES[12].code)
        .add_string_choice(LANGUAGES[13].name, LANGUAGES[13].code)
        .add_string_choice(LANGUAGES[14].name, LANGUAGES[14].code)
        .add_string_choice(LANGUAGES[15].name, LANGUAGES[15].code)
        .add_string_choice(LANGUAGES[16].name, LANGUAGES[16].code)
        .add_string_choice(LANGUAGES[17].name, LANGUAGES[17].code)
        .add_string_choice(LANGUAGES[18].name, LANGUAGES[18].code)
        .add_string_choice(LANGUAGES[19].name, LANGUAGES[19].code)
        .add_string_choice(LANGUAGES[20].name, LANGUAGES[20].code)
        .add_string_choice(LANGUAGES[21].name, LANGUAGES[21].code)
        .add_string_choice(LANGUAGES[22].name, LANGUAGES[22].code)
        .add_string_choice(LANGUAGES[23].name, LANGUAGES[23].code)
        .add_string_choice(LANGUAGES[24].name, LANGUAGES[24].code);

    CreateCommand::new("translate")
        .description("Translate a text")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "text",
                "The text you wish to translate."
            ).required(true)
        )
        .add_option(from)
        .add_option(to)
}