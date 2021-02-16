use serenity::model::channel::Message;
pub fn author_is_bot(msg: &Message) -> bool {
    msg.author.bot
}

pub fn send_via_dm(msg: &Message) -> bool {
    match msg.guild_id {
        Some(_guild_id) => false,
        _ => true
    }
}