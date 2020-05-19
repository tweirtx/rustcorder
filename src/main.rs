use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::fs;

struct Handler;
impl EventHandler for Handler {
    fn message(&self, ctx: Context, message: Message) {
        if message.content.starts_with("r!record") {
            //if message.author
            message.channel_id.say(ctx.http, "You said ".to_owned() + message.content.as_str());
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
