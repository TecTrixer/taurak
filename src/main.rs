use dotenv;
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

mod commands;
use crate::commands::*;

mod checks;
use crate::checks::*;

#[async_trait]
impl EventHandler for Handler {
    // For each received message do this:
    async fn message(&self, ctx: Context, msg: Message) {
        // Check if message is either from me or another bot
        if !author_is_bot(&msg) {
            // Check if message is sent via direct message
            if !send_via_dm(&msg) || author_is_owner(&msg) {
                // TODO: add variable prefix (needs to be stored in data)
                let parsed_message = parse_command(&msg, "t".into());
                if parsed_message.is_command {
                    let functions = get_commands();
                    match functions.get(&parsed_message.command.expect("Err parsing the command")) {
                        Some(function) => {
                            function(
                                ctx,
                                msg,
                                ParsedCommand {
                                    is_command: false,
                                    command: None,
                                    args: parsed_message.args,
                                },
                            )
                            .await;
                        }
                        _ => {
                            if let Err(why) = msg
                                .channel_id
                                .say(&ctx.http, "Sorry, I do not know what to do now :(")
                                .await
                            {
                                println!("Error sending message: {:?}", why);
                            }
                        }
                    }
                }
            // TODO: handle message xp when message is no command
            }
            // Sent via direct message
            else {
                let functions = get_commands();
                match functions.get("dm_not_implemented") {
                    Some(function) => {
                        function(
                            ctx,
                            msg,
                            ParsedCommand {
                                is_command: false,
                                command: None,
                                args: None,
                            },
                        )
                        .await;
                    }
                    _ => println!("No command found"),
                }
            }
        }
    }

    // After connecting successfully with discord do this:
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with discord bot token in the environment.
    dotenv::from_filename("secrets.env").expect("No secret token found!");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // TODO: add presence update with funny message

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
    // TODO: handle process exit so the presence is offline
}
