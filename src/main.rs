use std::env;
use dotenv::dotenv;

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
        if !author_is_bot(&msg) {
        let functions = get_commands();
        let args = msg.content.split_whitespace().next().unwrap();
        match functions.get(args) {
            Some(function) => {function(ctx,msg).await;},
            _ => println!("No command found")
        }
    }}

    // After connecting successfully with discord do this:
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with discord bot token in the environment.
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
