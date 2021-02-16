use serenity::model::channel::Message;
pub fn author_is_bot(msg: &Message) -> bool {
    msg.author.bot
}