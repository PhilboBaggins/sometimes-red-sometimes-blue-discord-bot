use serenity::{
    model::{channel::Message, gateway::Ready},
    builder::CreateMessage,
    utils::Colour,
    prelude::*,
};

use rand::thread_rng;
use rand::seq::SliceRandom;

const MY_CLIENT_ID: u64 = 713872191682510909;

struct Handler;

// TODO: Consider using rand_derive
enum Colours {
    Red,
    Blue
}

impl Colours {
    fn to_string(&self) -> &'static str {
        match self {
            Colours::Red => "Red",
            Colours::Blue => "Blue",
        }
    }

    fn to_colour(&self) -> Colour {
        match self {
            Colours::Red => Colour::from_rgb(255, 0, 0),
            Colours::Blue => Colour::from_rgb(0, 0, 255),
        }
    }
}

fn gen_colour_message(message: CreateMessage) -> CreateMessage {
    let choices = [Colours::Red, Colours::Blue];
    let mut rng = thread_rng();
    let choice = choices.choose(&mut rng).unwrap();
    message
        //.content(choice.to_string())
        .embed(|e| e
            .title(choice.to_string())
            .colour(choice.to_colour()))
}

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    fn message(&self, _ctx: Context, msg: Message) {
        if !msg.is_own() && (msg.is_private() || msg.mentions_user_id(MY_CLIENT_ID)) {
            if let Err(why) = msg.channel_id.send_message(gen_colour_message) {
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
