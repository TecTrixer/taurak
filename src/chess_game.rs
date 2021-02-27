use crate::checks::ParsedCommand;
use serenity::{model::channel::Message, prelude::*};

pub async fn render_board(ctx: Context, msg: Message, _args: ParsedCommand) -> () {
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