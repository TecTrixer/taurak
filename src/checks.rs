use serenity::{cache::Cache, model::channel::Message, prelude::*,};
use std::env;
use censor::*;


#[derive(Clone, Debug)]
pub struct ParsedCommand {
    pub is_command: bool,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
}

pub fn author_is_bot(msg: &Message) -> bool {
    msg.author.bot
}

// TODO: remove if not used in the future, is the difference between taurak & other bots important?
pub async fn _author_is_taurak(msg: Message, cache: &Cache) -> bool {
    msg.is_own(cache).await
}

pub fn send_via_dm(msg: &Message) -> bool {
    match msg.guild_id {
        Some(_guild_id) => false,
        _ => true,
    }
}

pub fn parse_command(msg: &Message, prefix: String) -> ParsedCommand {
    let mut parsed_message = msg.content.split_whitespace();
    let first_word = match parsed_message.next() {
        Some(prefix) => prefix,
        _ => "".into(),
    };
    if first_word == prefix {
        let command: Option<String> = match parsed_message.next() {
            Some(command) => Some(command.into()),
            _ => Some("no command".into()),
        };

        let mut args: Vec<String> = vec![];
        let mut more_args: bool = true;
        let mut arg: String = "".into();
        match parsed_message.next() {
            Some(argument) => {
                arg = argument.into();
            }
            _ => {
                more_args = false;
            }
        };
        while more_args {
            args.push(arg.clone());
            match parsed_message.next() {
                Some(argument) => {
                    arg = argument.into();
                }
                _ => {
                    more_args = false;
                }
            };
        }
        return ParsedCommand {
            is_command: true,
            command: command,
            args: Some(args),
        };
    } else {
        return ParsedCommand {
            is_command: false,
            command: None,
            args: None,
        };
    }
}

pub async fn _author_has_role(msg: Message, role_name: String, cache: &Cache, ctx: Context) -> bool {
    let guild = msg.guild(cache).await.expect("Guild was not in cache");
    let guild_id = guild.id;
   let role_map = guild.roles;
   let mut role_id: Option<&serenity::model::id::RoleId> = None;
   if role_map.iter().count() == 0 {
       return false;
   }
   for (key, var) in role_map.iter() {
       if var.name.to_ascii_lowercase() == role_name.to_ascii_lowercase() {
            role_id = Some(key);
            break;
       }
   }
   match role_id {
       Some(role_id) => {if msg.author.has_role(ctx.http, guild_id, role_id).await.expect("Err getting role") {
        true
    }
    else {
        false
    }}
    _ => {return false;}
   }
   
}

pub async fn author_has_permission(msg: Message, permission: serenity::model::permissions::Permissions, cache: &Cache, ctx: Context) -> bool {
    let guild = msg.guild(cache).await.expect("Guild was not in cache");
    let guild_id = guild.id;
   let role_map = guild.roles;
   if role_map.iter().count() == 0 {
       return false;
   }
   for (key, var) in role_map.iter() {
       if var.has_permission(permission) {
        if msg.author.has_role(ctx.clone().http, guild_id, key).await.expect("Err getting role") {
            return true;
        }
       }
   }
   return false;
}

pub fn author_is_owner(msg: &Message) -> bool {
    if msg.author.id.as_u64() == &env::var("OWNER_ID").expect("No OWNER_ID in environment").parse::<u64>().expect("OWNER_ID was not a number!") {
        true
    }
    else {
        false
    }
}

pub fn has_profanity_content(msg: &Message) -> bool {
    let censor = Standard + Zealous + Sex;
    censor.check(&msg.content)
}
