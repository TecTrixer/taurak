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
    // functions.insert("lyr".into(), lyr);
    // functions.insert("lyrics".into(), lyr);
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
    functions.insert("offline".into(), offline);
    functions.insert("online".into(), online);
    functions.insert("chess".into(), chess);
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

pub fn _lyr(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(_lyr_async(ctx, msg, args))
}

async fn _lyr_async(ctx: Context, msg: Message, args: ParsedCommand) {
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
pub fn offline(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(offline_async(ctx, msg, args))
}

async fn offline_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if author_has_permission(msg.clone(), serenity::model::permissions::Permissions::ADMINISTRATOR, &ctx.clone().cache, ctx.clone()).await {
    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "It's getting dark...",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
    ctx.clone().invisible().await;
}
    else {
        if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "Sorry, only for admins :(",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
    }
}

pub fn online(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(online_async(ctx, msg, args))
}

async fn online_async(ctx: Context, msg: Message, _args: ParsedCommand) {
    if author_has_permission(msg.clone(), serenity::model::permissions::Permissions::ADMINISTRATOR, &ctx.clone().cache, ctx.clone()).await {
    if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "Ahh, the sun is rising...",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
    ctx.clone().online().await;
}
    else {
        if let Err(why) = msg
        .channel_id
        .say(
            &ctx.http,
            "Sorry, only for admins :(",
        )
        .await
    {
        println!("Error sending message: {:?}", why);
    }
    }
}


// TODO: add chessrs and implement displaying a board with figures && later also adding possibility to move
pub fn chess(
    ctx: Context,
    msg: Message,
    args: ParsedCommand,
) -> Pin<Box<dyn Future<Output = ()> + std::marker::Send>> {
    Box::pin(chess_async(ctx, msg, args))
}

async fn chess_async(ctx: Context, msg: Message, args: ParsedCommand) {
    if match args.args {Some(v) => v.len() == 0, _ => true} {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Ahh, you want to play chess...\nI need some more information to start a match:\nIf you want to play against a bot enter ```t chess bot```").await {
        println!("Error sending message: {:?}", why);
    }
}else{
    let mut board = "".to_owned();
    for x in 0..8 {
        for i in 0..8{
            if ((i & 1) != 0 && (x & 1) != 0) || ((i & 1) == 0 && (x & 1) == 0){
                board.push_str("<:de:814922667081596980>");
            }
            else {
                board.push_str("<:le:814922666967826474>");
            }
        }
        board.push_str("\n");
    }
    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m | {
        m.embed(|e | {
          e.title("Basic chess board:");
          e.description(format!("{}", board));
          
          return e;
        });
        
        return m;
      }
    ).await{println!("Error sending message: {:?}", why);}
}
}


// TODO: add help command (maybe automatic implementation)
