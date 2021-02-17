use crate::checks::ParsedCommand;
use serenity::{model::channel::Message, prelude::*};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use crate::checks::author_has_permission;

pub fn get_commands() -> HashMap<
    String,
    fn(Context, Message, ParsedCommand) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>,
> {
    let mut functions: HashMap<
        String,
        fn(
            Context,
            Message,
            ParsedCommand,
        ) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>,
    > = HashMap::<
        String,
        fn(
            Context,
            Message,
            ParsedCommand,
        ) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>>,
    >::new();
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
    functions.insert("admin".into(), admin);
    functions.insert("am_i_admin".into(), admin);
    functions.insert("Admin".into(), admin);
    functions.insert("Administrator".into(), admin);
    functions.insert("administrator".into(), admin);
    // TODO: Maybe handle aliases in a better more efficient way

    return functions;
}

pub fn ping(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(ping_async(ctx, msg, args))
}

async fn ping_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        println!("Error sending message: {:?}", why);
    }
}
pub fn dm_not_implemented(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(dm_not_implemented_async(ctx, msg, args))
}

async fn dm_not_implemented_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "Sorry, but I do not have any functions for DM's yet.",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}

pub fn no_command(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(no_command_async(ctx, msg, args))
}

async fn no_command_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "You need to specify a command after the prefix, otherwise I do not know what to do.",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}

pub fn test(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(test_async(ctx, msg, args))
}

async fn test_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Hello, im online :)").await {
        println!("Error sending message: {:?}", why);
    }
}

pub fn lyr(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(lyr_async(ctx, msg, args))
}

async fn lyr_async(ctx: Context, msg: Message, args: ParsedCommand) {
    use std::process::Command;

    let song = match args.args {
        Some(args) => args.join(" "),
        _ => "".into(),
    };

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("lyr-no-prompt query {}", song))
        .output()
        .expect("Error using the lyr command");

    let hello =
        String::from_utf8(output.stdout).expect("Error when converting command output to string");
    if hello.chars().count() == 0 {
        if let Err(why) = msg
            .channel_id
            .say(
                &ctx.http,
                "You need to specify a song otherwise I do not know what to search for :(",
            )
            .await
        {
            println!("Error sending message: {:?}", why);
        }
    } else {
        let paragraphs: Vec<&str> = hello.split("\n\n").collect();
        for paragraph in paragraphs {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, format!("{}", paragraph))
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

pub fn lol(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(lol_async(ctx, msg, args))
}

async fn lol_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "When I was young we used to say ROFL instead of lol.",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}
pub fn xd(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(xd_async(ctx, msg, args))
}

async fn xd_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "You can also use emojis, they are better for showing your emotions",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}
pub fn soeren(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(soeren_async(ctx, msg, args))
}

async fn soeren_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if let Err(why) = msg
        .channel_id
        .say(&ctx.http, "Scheel\nDer einzig Wahre...")
        .await
    {
        println!("Error sending message: {:?}", why);
    }
}

pub fn admin(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(admin_async(ctx, msg, args))
}

async fn admin_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if author_has_permission(msg.clone(), serenity::model::permissions::Permissions::ADMINISTRATOR, &ctx.clone().cache, ctx.clone()).await {
    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "Yes, you are the admin, my Lord",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }}
    else {
        if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "No, you do not possess the power of god",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
    }
}

// TODO: add help command (maybe automatic implementation)
