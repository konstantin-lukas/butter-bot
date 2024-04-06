use std::collections::HashMap;
use serenity::all::{ResolvedOption, ResolvedValue};

pub(crate) type CommandArgs = HashMap<String, String>;
pub fn get_command_args(options: &[ResolvedOption<'_>]) -> CommandArgs {
    let mut args = HashMap::new();
    for o in options {
        if let ResolvedOption { value: ResolvedValue::String(str), .. } = o {
            args.insert(o.name.to_string(), str.to_string());
        }
    }
    args
}