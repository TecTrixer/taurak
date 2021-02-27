use serenity::{model::channel::Message, prelude::*};

pub async fn render_board(ctx: Context, msg: Message, position: &String) -> () {
    let mut board = "".to_owned();
    let fen_command = position.chars();
    let mut column: i8 = 0;
    let mut row: i8 = 0;
    for command in fen_command {
        match command {
            'r' => {if is_square_dark(&column, &row) {board.push_str("<:rdd:814922667035459605>");}else {board.push_str("<:rdl:814922666871095366>");}},
            'n' => {if is_square_dark(&column, &row) {board.push_str("<:ndd:814922667282923581>");}else {board.push_str("<:ndl:814922666695065611>");}},
            'b' => {if is_square_dark(&column, &row) {board.push_str("<:bdd:814922666968350731>");}else {board.push_str("<:bdl:814922667190648932>");}},
            'q' => {if is_square_dark(&column, &row) {board.push_str("<:qdd:814922667223285771>");}else {board.push_str("<:qdl:814922666988535870>");}},
            'k' => {if is_square_dark(&column, &row) {board.push_str("<:kdd:814922667340857384>");}else {board.push_str("<:kdl:814922667085135943>");}},
            'p' => {if is_square_dark(&column, &row) {board.push_str("<:pdd:814922666896654379>");}else {board.push_str("<:pdl:814922666645520458>");}},
            'P' => {if is_square_dark(&column, &row) {board.push_str("<:pld:814922667307565116>");}else {board.push_str("<:pll:814922666993647616>");}},
            'R' => {if is_square_dark(&column, &row) {board.push_str("<:rld:814922667127341056>");}else {board.push_str("<:rll:814922627550150687>");}},
            'N' => {if is_square_dark(&column, &row) {board.push_str("<:nld:814922666837409884>");}else {board.push_str("<:nll:814922666682613831>");}},
            'B' => {if is_square_dark(&column, &row) {board.push_str("<:bld:814922667110039594>");}else {board.push_str("<:bll:814922667151982652>");}},
            'Q' => {if is_square_dark(&column, &row) {board.push_str("<:qld:814922667261296690>");}else {board.push_str("<:qll:814922667081072720>");}},
            'K' => {if is_square_dark(&column, &row) {board.push_str("<:kld:814922667345444984>");}else {board.push_str("<:kll:814922667115282432>");}},
            '/' => {board.push_str("\n"); column = -1; row += 1;},
            ' ' => break,
            number => {for _ in 0..(number as u8 - '0' as u8) {if is_square_dark(&column, &row) {board.push_str("<:de:814922667081596980>");}else {board.push_str("<:le:814922666967826474>");} column += 1;}column -= 1;},
        }
        column += 1;
    }
    if let Err(why_not) = msg.channel_id.send_message(&ctx.http, |m | {
        m.embed(|e | {
          e.title("Basic chess board:");
          e.description(format!("{}", board));
          
          return e;
        });
        
        return m;
      }
    ).await{if let Err(why) = msg.channel_id.say(&ctx.http, format!("Sorry, there was an error parsing your FEN string:\n```{}```", why_not)).await {
        println!("Error sending message: {:?}", why);
    }}
}

fn is_square_dark(column: &i8 , row: &i8) -> bool {
    if (column & 1) == (row & 1){
        false
    }
    else {
        true
    }
}