//! Requires the "client", "standard_framework", and "voice" features be enabled
//! in your Cargo.toml, like so:
//!
//! ```toml
//! [dependencies.serenity]
//! git = "https://github.com/serenity-rs/serenity.git"
//! features = ["client", "standard_framework", "voice"]
//! ```
use std::{sync::Arc, fs};

use serenity::{
    async_trait,
    client::{bridge::voice::ClientVoiceManager, Client, Context, EventHandler},
    framework::{
        standard::{
            macros::{group},
        },
    },
    model::{channel::Message, gateway::Ready, id::ChannelId},
    prelude::*,
    voice::AudioReceiver,
    Result as SerenityResult,
};

struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

struct Handler;

struct Receiver;

#[async_trait]
impl AudioReceiver for Receiver {
    async fn speaking_update(&self, _ssrc: u32, _user_id: u64, _speaking: bool) {
        // You can implement logic here so that you can differentiate users'
        // SSRCs and map the SSRC to the User ID and maintain a state in
        // `Receiver`. Using this map, you can map the `ssrc` in `voice_packet`
        // to the user ID and handle their audio packets separately.
    }

    async fn voice_packet(
        &self,
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

    async fn client_connect(&self, _ssrc: u32, _user_id: u64) {
        // You can implement your own logic here to handle a user who has joined the
        // voice channel e.g., allocate structures, map their SSRC to User ID.
    }

    async fn client_disconnect(&self, _user_id: u64) {
        // You can implement your own logic here to handle a user who has left the
        // voice channel e.g., finalise processing of statistics etc.
        // You will typically need to map the User ID to their SSRC; observed when
        // speaking or connecting.
    }
}

#[group]
struct General;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let token = fs::read_to_string("token.txt").expect("token.txt read error");
    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Obtain a lock to the data owned by the client, and insert the client's
    // voice manager into it. This allows the voice manager to be accessible by
    // event handlers and framework commands.
    {
        let mut data = client.data.write().await;
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
    }
}

/*#[command]
async fn join(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let connect_to = match args.single::<u64>() {
        Ok(id) => ChannelId(id),
        Err(_) => {
            check_msg(msg.reply(ctx, "Requires a valid voice channel ID be given").await);

            return Ok(());
        },
    };

    let guild_id = match ctx.cache.guild_channel_field(msg.channel_id, |channel| channel.guild_id).await {
        Some(id) => id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "DMs not supported").await);

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().await
        .get::<VoiceManager>().cloned().expect("Expected VoiceManager in TypeMap.");
    let mut manager = manager_lock.lock().await;

    if let Some(handler) = manager.join(guild_id, connect_to) {
        handler.listen(Some(Arc::new(Receiver::new())));
        check_msg(msg.channel_id.say(&ctx.http, &format!("Joined {}", connect_to.mention())).await);
    } else {
        check_msg(msg.channel_id.say(&ctx.http, "Error joining the channel").await);
    }

    Ok(())
}

async fn leave(ctx: &mut Context, msg: &Message) -> CommandResult {
    let guild_id = match ctx.cache.guild_channel_field(msg.channel_id, |channel| channel.guild_id).await {
        Some(id) => id,
        None => {
            check_msg(msg.channel_id.say(&ctx.http, "DMs not supported").await);

            return Ok(());
        },
    };

    let manager_lock = ctx.data.read().await.get::<VoiceManager>().cloned()
        .expect("Expected VoiceManager in TypeMap.");
    let mut manager = manager_lock.lock().await;
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        manager.remove(guild_id);

        check_msg(msg.channel_id.say(&ctx.http,"Left voice channel").await);
    } else {
        check_msg(msg.reply(ctx, "Not in a voice channel").await);
    }

    Ok(())
}

async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http,"Pong!").await);

    Ok(())
}*/

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, mut ctx: Context, message: Message) {
        let manager_lock = ctx.data.read().await.get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
        let mut manager = manager_lock.lock();
        if message.content.starts_with("r!record") {
            let voiceid = message.content.trim_start_matches("r!record ");
            print!("{}", voiceid);
            if voiceid =="r!record" {
                message.channel_id.say(&ctx.http, "No ID given!").await.expect("Error sending msg!");
                return;
            }
            let id_as_int = voiceid.parse::<u64>();
            match id_as_int {
                Ok(x) => {
                    message.channel_id.say(&ctx.http, "Voice ID: ".to_owned() + voiceid).await.expect("Error sending msg!");
                    let guild = message.guild_id;
                    match guild {
                        Some(i) => {
                            if let Some(handler) = manager.await.join(i, ChannelId(x)) {
                                handler.listen(Some(Arc::new(VoiceManager::new.await)));
                                println!("right track, wrong train");
                            }
                            else {
                                println!("oh darn");
                            }
                        },
                        None => {
                            message.channel_id.say(&ctx.http, "Hey, wait a minute. This is a DM!").await.expect("Error replying");
                        }
                    }
                }
                Err(_id_as_int) => {
                    message.channel_id.say(&ctx.http, "Failed to parse ID!").await.expect("Error sending msg!");
                    return;
                }
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
