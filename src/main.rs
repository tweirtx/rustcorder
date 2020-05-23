use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    client::{bridge::voice::ClientVoiceManager, Client, Context, EventHandler},
};
use std::fs;
use serenity::model::id::ChannelId;

struct Handler {
    voice_manager: ClientVoiceManager,
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, message: Message) {
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
                    Handler::join(&self.voice_manager, ChannelId(x));
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
    let mut dc = Client::new(token, Handler { voice_manager: (Client.voice_manager) }).expect("Creating client failed");
    dc.start().expect("Error starting");
}
