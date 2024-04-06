use std::env;
use std::sync::Arc;
use serenity::all::{CreateMessage, Http, ChannelId, UserId};


pub async fn dm_admin(http: Arc<Http>, message: &str) {
    let admin_id = env::var("ADMIN_ID")
        .unwrap_or_default()
        .parse()
        .unwrap_or_default();

    let admin = match http.get_user(UserId::new(admin_id)).await {
        Ok(x) => x,
        Err(e) => {
            println!("Error creating DM channel: {:?}", e);
            return;
        }
    };

    if let Err(e) = admin.create_dm_channel(&http).await {
        println!("Error creating DM channel: {:?}", e);
        return;
    }

    if let Err(e) = admin.dm(&http, CreateMessage::new().content(message)).await {
        println!("Error sending message: {:?}", e);
    }
}
pub async fn post_to_dbd_channel(http: Arc<Http>, message: &str) {

    let dbd_channel_id = env::var("DBD_CHANNEL")
        .unwrap_or_default()
        .parse();

    if let Ok(channel) = dbd_channel_id {
        let dbd_channel = ChannelId::new(channel);

        let mut remaining_message = message;
        while !remaining_message.is_empty() {
            let chunk = &remaining_message[..std::cmp::min(remaining_message.len(), 2000)];
            remaining_message = &remaining_message[chunk.len()..];
            if let Err(why) = dbd_channel.say(&http, chunk).await {
                println!("Error sending message: {:?}", why);
                return;
            }
        }
    }
}
