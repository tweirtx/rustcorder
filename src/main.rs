use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;
impl EventHandler for Handler {
    fn message(&self, ctx: Context, message: Message) {
        if message.content.starts_with("r!record") {
            if message.author
            message.channel_id.say(ctx.http, "You said ".to_owned() + message.content.as_str());
        }

    }
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}


fn main() {
    let mut dc = Client::new("", Handler).expect("Creating client failed");
    dc.start();
    println!("Hello, world!");
}
