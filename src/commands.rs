use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use serenity::{
    model::channel::Message,
    prelude::*,
};

pub fn get_commands() -> HashMap::<String, fn(Context, Message) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>> {
    let mut functions: HashMap::<String, fn(Context, Message) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>> = HashMap::<String, fn(Context, Message) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>>::new();
    functions.insert("ping".into(), ping);
    functions.insert("dm_not_implemented".into(), dm_not_implemented);
    functions.insert("no command".into(), no_command);

    return functions;
}


pub fn ping(ctx: Context, msg: Message) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>{
    Box::pin(ping_async(ctx, msg))
}

async fn ping_async(ctx: Context, msg: Message){
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
    }
}
pub fn dm_not_implemented(ctx: Context, msg: Message) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>{
    Box::pin(dm_not_implemented_async(ctx, msg))
}

async fn dm_not_implemented_async(ctx: Context, msg: Message){
    if let Err(why) = msg.channel_id.say(&ctx.http, "Sorry, but I do not have any functions for DM's yet.").await {
        println!("Error sending message: {:?}", why);
    }
}

pub fn no_command(ctx: Context, msg: Message) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>{
    Box::pin(no_command_async(ctx, msg))
}

async fn no_command_async(ctx: Context, msg: Message){
    if let Err(why) = msg.channel_id.say(&ctx.http, "You need to specify a command after the prefix, otherwise I do not know what to do.").await {
        println!("Error sending message: {:?}", why);
    }
}

