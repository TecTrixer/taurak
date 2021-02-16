use serenity::{model::channel::Message, cache::Cache};
pub fn author_is_bot(msg: &Message) -> bool {
    msg.author.bot
}

pub async fn author_is_taurak(msg: Message, cache: &Cache) -> bool {
    msg.is_own(cache).await
}

pub fn send_via_dm(msg: &Message) -> bool {
    match msg.guild_id {
        Some(_guild_id) => false,
        _ => true
    }
}

// pub async fn author_has_role(msg: Message, role_name: String, cache: &Cache) -> bool {
//    true 
// }