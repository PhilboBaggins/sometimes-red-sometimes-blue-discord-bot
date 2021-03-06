use serenity::{
    model::{channel::Message, gateway::Ready, id::UserId},
    prelude::*,
    utils::Colour,
};

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::env;
use std::fmt;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct Handler {
    my_id: Arc<Mutex<UserId>>,
}

// TODO: Consider using rand_derive
enum Colours {
    Red,
    Blue,
}

impl fmt::Display for Colours {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Colours::Red => write!(f, "{}", "Red"),
            Colours::Blue => write!(f, "{}", "Blue"),
        }
    }
}

impl Colours {
    fn to_colour(&self) -> Colour {
        match self {
            Colours::Red => Colour::from_rgb(255, 0, 0),
            Colours::Blue => Colour::from_rgb(0, 0, 255),
        }
    }
}

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Save my user ID so I can check if received messages are mentioning me
        let mut my_id = self.my_id.lock().unwrap();
        *my_id = ready.user.id;
    }

    fn message(&self, ctx: Context, msg: Message) {
        if !msg.is_own(ctx.cache) && (msg.is_private() || msg.mentions_user_id(*self.my_id.lock().unwrap()))
        {
            let ret = msg.channel_id.send_message(&ctx.http, |message| {
                let choices = [Colours::Red, Colours::Blue];
                let mut rng = thread_rng();
                let choice = choices.choose(&mut rng).unwrap();
                message.embed(|e| e.title(choice).colour(choice.to_colour()));
                message
            });
            if let Err(why) = ret {
                eprintln!("Error sending message: {:?}", why);
            }
        }
    }
}

pub fn main() {
    let handler: Handler = Default::default();

    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), handler)
        .expect("Error creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start() {
        eprintln!("An error occurred while running the client: {:?}", why);
    }
}
