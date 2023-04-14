use std::{env, sync::Arc, thread};

use dotenv::dotenv;
use serenity::{model::prelude::PartialGuild, prelude::GatewayIntents, Client};

use banlog::{watch, Handler, CHANNEL_ID, GUILD_ID};

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to get bot token");
    let token = env::var("DISCORD_TOKEN").expect("Token not found in the env file");

    let intents = GatewayIntents::GUILD_MESSAGES;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client");

    if let Err(why) = client.start().await {
        println!("Failed to start bot, error:'{why}");
    }
}
