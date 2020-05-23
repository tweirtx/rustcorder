use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    client::{bridge::voice::ClientVoiceManager, Client, Context, EventHandler},
};
use std::fs;
use serenity::model::id::ChannelId;
use std::sync::Arc;

struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, message: Message) {
        let manager_lock = ctx.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
        let mut manager = manager_lock.lock();
        if message.content.starts_with("r!record") {
            let voiceid = message.content.trim_start_matches("r!record ");
            print!("{}", voiceid);
            if voiceid =="r!record" {
                message.channel_id.say(&ctx.http, "No ID given!").expect("Error sending msg!");
                return;
            }
            let id_as_int = voiceid.parse::<u64>();
            match id_as_int {
                Ok(x) => {
                    message.channel_id.say(&ctx.http, "Voice ID: ".to_owned() + voiceid).expect("Error sending msg!");
                    let guild = message.guild_id;
                    match guild {
                        Some(i) => {
                            manager.join(i, ChannelId(x));
                        },
                        None => {
                            message.channel_id.say(&ctx.http, "Hey, wait a minute. This is a DM!");
                        }
                    }
                }
                Err(id_as_int) => {
                    message.channel_id.say(&ctx.http, "Failed to parse ID!").expect("Error sending msg!");
                    return;
                }
            }
        }
    }
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


fn main() {
    let token = fs::read_to_string("token.txt").expect("token.txt read error");
    let mut dc = Client::new(token, Handler).expect("Creating client failed");
    {
        let mut data = dc.data.write();
        data.insert::<VoiceManager>(Arc::clone(&dc.voice_manager));
    }
    dc.start().expect("Error starting");
}
