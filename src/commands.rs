use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use serenity::{
    model::channel::Message,
    prelude::*,
};
use crate::checks::ParsedCommand;

pub fn get_commands() -> HashMap::<String, fn(Context, Message, ParsedCommand) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>> {
    let mut functions: HashMap::<String, fn(Context, Message, ParsedCommand) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>> = HashMap::<String, fn(Context, Message, ParsedCommand) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>>::new();
    functions.insert("ping".into(), ping);
    functions.insert("dm_not_implemented".into(), dm_not_implemented);
    functions.insert("no command".into(), no_command);
    functions.insert("test".into(), test);
    functions.insert("lyr".into(), lyr);
    functions.insert("lyrics".into(), lyr);
    functions.insert("lol".into(), lol);
    functions.insert("xD".into(), xd);
    functions.insert("xd".into(), xd);
    functions.insert("XD".into(), xd);
    functions.insert("Sören".into(), soeren);
    functions.insert("Sö".into(), soeren);
    functions.insert("Söse".into(), soeren);

    return functions;
}


pub fn ping(ctx: Context, msg: Message, args: ParsedCommand) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>{
    Box::pin(ping_async(ctx, msg, args))
}

async fn ping_async(ctx: Context, msg: Message, _args: ParsedCommand){
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
    }
}
pub fn dm_not_implemented(ctx: Context, msg: Message, args: ParsedCommand) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>{
    Box::pin(dm_not_implemented_async(ctx, msg, args))
}

async fn dm_not_implemented_async(ctx: Context, msg: Message, _args: ParsedCommand){
    if let Err(why) = msg.channel_id.say(&ctx.http, "Sorry, but I do not have any functions for DM's yet.").await {
        println!("Error sending message: {:?}", why);
    }
}

pub fn no_command(ctx: Context, msg: Message, args: ParsedCommand) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>{
    Box::pin(no_command_async(ctx, msg, args))
}

async fn no_command_async(ctx: Context, msg: Message, _args: ParsedCommand){
    if let Err(why) = msg.channel_id.say(&ctx.http, "You need to specify a command after the prefix, otherwise I do not know what to do.").await {
        println!("Error sending message: {:?}", why);
    }
}

pub fn test(ctx: Context, msg: Message, args: ParsedCommand) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>{
    Box::pin(test_async(ctx, msg, args))
}

async fn test_async(ctx: Context, msg: Message, _args: ParsedCommand){
    if let Err(why) = msg.channel_id.say(&ctx.http, "Hello, im online :)").await {
        println!("Error sending message: {:?}", why);
    }
}

pub fn lyr(ctx: Context, msg: Message, args: ParsedCommand) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>{
    Box::pin(lyr_async(ctx, msg, args))
}

async fn lyr_async(ctx: Context, msg: Message, args: ParsedCommand){
    use std::process::{Command, Stdio};
    use std::io::Write;
    let song = match args.args {
        Some(args) => args.join(" "),
        _ => "".into(),
    };


let mut child = Command::new("sh").arg("-c").arg(format!("lyr query {}", song)).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("Error using the lyr command");

{
    let stdin = child.stdin.as_mut().expect("Failed to open stdin");
    stdin.write_all("1\n".as_bytes()).expect("Failed to write to stdin");
}

let output = child.wait_with_output().expect("Failed to read stdout");

let hello = String::from_utf8(output.stdout).expect("Error when converting command output to string");
    if let Err(why) = msg.channel_id.say(&ctx.http, format!("{}", hello)).await {
        println!("Error sending message: {:?}", why);
    }
}

pub fn lol(ctx: Context, msg: Message, args: ParsedCommand) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>{
    Box::pin(lol_async(ctx, msg, args))
}

async fn lol_async(ctx: Context, msg: Message, _args: ParsedCommand){
    if let Err(why) = msg.channel_id.say(&ctx.http, "When I was young we used to say ROFL instead of lol.").await {
        println!("Error sending message: {:?}", why);
    }
}
pub fn xd(ctx: Context, msg: Message, args: ParsedCommand) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>{
    Box::pin(xd_async(ctx, msg, args))
}

async fn xd_async(ctx: Context, msg: Message, _args: ParsedCommand){
    if let Err(why) = msg.channel_id.say(&ctx.http, "You can also use emojis, they are better for showing your emotions").await {
        println!("Error sending message: {:?}", why);
    }
}
pub fn soeren(ctx: Context, msg: Message, args: ParsedCommand) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>{
    Box::pin(soeren_async(ctx, msg, args))
}

async fn soeren_async(ctx: Context, msg: Message, _args: ParsedCommand){
    if let Err(why) = msg.channel_id.say(&ctx.http, "Scheel\nDer einzig Wahre...").await {
        println!("Error sending message: {:?}", why);
    }
}

