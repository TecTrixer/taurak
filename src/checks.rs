use serenity::{model::channel::Message, cache::Cache};

#[derive(Clone, Debug)]
pub struct ParsedCommand {
    pub is_command: bool,
    pub command: Option<String>,
    pub args: Option<Vec<String>>
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
        _ => true
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
            Some(argument) => {arg = argument.into();},
            _ => {more_args = false;},
        };
        while more_args {
            args.push(arg.clone());
            match parsed_message.next() {
                Some(argument) => {arg = argument.into();},
                _ => {more_args = false;},
            };
        }
        return ParsedCommand{is_command: true, command: command, args: Some(args)}
    }
    else {
        return ParsedCommand{is_command: false, command: None, args: None}
    }
}

// TODO: implement this check

// pub async fn author_has_role(msg: Message, role_name: String, cache: &Cache) -> bool {
//    true 
// }