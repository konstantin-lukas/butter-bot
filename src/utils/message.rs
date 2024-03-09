use std::sync::Arc;
use serenity::all::{CreateMessage, Http, User, ChannelId};


pub struct MessageHandler {
    http: Arc<Http>,
    admin: User,
    dbd_channel: ChannelId
}


impl MessageHandler {
    pub fn new(http: Arc<Http>, admin: User, dbd_channel: ChannelId) -> Self {
        Self {
            http,
            admin,
            dbd_channel
        }
    }
    pub async fn dm_admin(&self, message: &str) {
        if let Err(why) = self.admin.create_dm_channel(&self.http).await {
            println!("Error creating DM channel: {:?}", why);
            return;
        }
        let dm = CreateMessage::new().content(message);
        if let Err(why) = self.admin.dm(&self.http, dm).await {
            println!("Error sending message: {:?}", why);
        }
    }
    pub async fn post_to_dbd_channel(&self, message: &str) {
        let mut remaining_message = message;
        while !remaining_message.is_empty() {
            let chunk = &remaining_message[..std::cmp::min(remaining_message.len(), 2000)];
            remaining_message = &remaining_message[chunk.len()..];
            if let Err(why) = self.dbd_channel.say(&self.http, chunk).await {
                println!("Error sending message: {:?}", why);
                return;
            }
        }
    }
}