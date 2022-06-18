#![forbid(unsafe_code)]

use serenity::{
    async_trait,
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

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // Save my user ID so I can check if received messages are mentioning me
        let mut my_id = self.my_id.lock().unwrap();
        *my_id = ready.user.id;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.is_own(ctx.cache)
            && (msg.is_private() || msg.mentions_user_id(*self.my_id.lock().unwrap()))
        {
            let ret = msg.channel_id.send_message(&ctx.http, |message| {
                let choices = [Colours::Red, Colours::Blue];
                let mut rng = thread_rng();
                let choice = choices.choose(&mut rng).unwrap();
                message.embed(|e| e.title(choice).colour(choice.to_colour()));
                message
            });
            if let Err(why) = ret.await {
                eprintln!("Error sending message: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let handler: Handler = Default::default();

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(handler)
        .await
        .expect("An error occurred while while creating the client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        eprintln!("An error occurred while running the client: {:?}", why);
    }
}
