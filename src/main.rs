#[macro_use] extern crate serenity;

use serenity::client::{Client, EventHandler};
use serenity::framework::standard::StandardFramework;

use std::env;

use rand::thread_rng;
use rand::seq::SliceRandom;

struct Handler;

impl EventHandler for Handler {}

pub fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("")) // set the bot's prefix to "~"
        .cmd("?", red_or_blue));

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(red_or_blue(_context, message) {
    let choices = ["red", "blue"];
    let mut rng = thread_rng();
    let choice = choices.choose(&mut rng).unwrap();
    let _ = message.reply(choice);
});
