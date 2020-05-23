use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use rand::thread_rng;
use rand::seq::SliceRandom;

const MY_CLIENT_ID: u64 = 713872191682510909;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    fn message(&self, _ctx: Context, msg: Message) {
        if !msg.is_own() && (msg.is_private() || msg.mentions_user_id(MY_CLIENT_ID)) {
            let choices = ["red", "blue"];
            let mut rng = thread_rng();
            let choice = choices.choose(&mut rng).unwrap();
            if let Err(why) = msg.channel_id.say(choice) {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

pub fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
