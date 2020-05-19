use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::fs;

struct Handler;
impl EventHandler for Handler {
    fn message(&self, ctx: Context, message: Message) {
        if message.content.starts_with("r!record") {
            let voiceid = message.content.trim_start_matches("r!record ");
            print!("{}", voiceid);
            if voiceid.eq("r!record") {
                message.channel_id.say(&ctx.http, "No ID given!");
                return;
            }
            message.channel_id.say(&ctx.http, "Voice ID: ".to_owned() + voiceid);
        }
    }
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


fn main() {
    let token = fs::read_to_string("token.txt").expect("token.txt read error");
    let mut dc = Client::new(token, Handler).expect("Creating client failed");
    dc.start();
}
