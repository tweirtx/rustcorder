use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    client::{bridge::voice::ClientVoiceManager, Client, Context, EventHandler},
};
use std::fs;
use serenity::model::id::ChannelId;
use std::sync::Arc;
use serenity::voice::AudioReceiver;

struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

struct Handler;

struct Receiver;

impl Receiver {
    pub fn new() -> Self {
        // You can manage state here, such as a buffer of audio packet bytes so
        // you can later store them in intervals.
        Self { }
    }
}

impl AudioReceiver for Receiver {
    fn speaking_update(&mut self, _ssrc: u32, _user_id: u64, _speaking: bool) {
        // You can implement logic here so that you can differentiate users'
        // SSRCs and map the SSRC to the User ID and maintain a state in
        // `Receiver`. Using this map, you can map the `ssrc` in `voice_packet`
        // to the user ID and handle their audio packets separately.
    }

    fn voice_packet(
        &mut self,
        ssrc: u32,
        sequence: u16,
        _timestamp: u32,
        _stereo: bool,
        data: &[i16],
        compressed_size: usize,
    ) {
        println!("Audio packet's first 5 bytes: {:?}", data.get(..5));
        println!(
            "Audio packet sequence {:05} has {:04} bytes (decompressed from {}), SSRC {}",
            sequence,
            data.len(),
            compressed_size,
            ssrc,
        );
    }
    fn client_connect(&mut self, _ssrc: u32, _user_id: u64) {
        // You can implement your own logic here to handle a user who has joined the
        // voice channel e.g., allocate structures, map their SSRC to User ID.
    }

    fn client_disconnect(&mut self, _user_id: u64) {
        // You can implement your own logic here to handle a user who has left the
        // voice channel e.g., finalise processing of statistics etc.
        // You will typically need to map the User ID to their SSRC; observed when
        // speaking or connecting.
    }
}

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
                            if let Some(handler) = manager.join(i, ChannelId(x)) {
                                handler.listen(Some(Box::new(Receiver::new())));
                            }
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
