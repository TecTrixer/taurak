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

    return functions;
}


pub fn ping(ctx: Context, msg: Message) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>{
    Box::pin(ping_async(ctx, msg))
}

async fn ping_async(ctx: Context, msg: Message){
    println!("Reached ping command");
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
    }
}
