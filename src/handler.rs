use serenity::builder::CreateEmbed;
use serenity::model::prelude::Embed;
use serenity::utils::{Colour, EmbedMessageBuilding};
use serenity::{
    async_trait,
    model::prelude::{Guild, GuildChannel, Message, Ready},
    prelude::{Context, EventHandler},
};
use std::fs;
use std::time::Duration;

use crate::FILE_PATH;

use super::watch;

use super::CHANNEL_ID;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, bot: Ready) {
        println!("Logged in as: {}#{}", bot.user.name, bot.user.discriminator);

        let gid = bot.guilds.get(0).unwrap().id;
        let guild = Guild::get(&ctx, gid).await.unwrap();

        let channels = guild.channels(&ctx).await.unwrap();
        let log_channel = channels.get(&CHANNEL_ID).unwrap();

        // --------------------

        let file_obj = fs::File::open(FILE_PATH).expect("Failed to open file");

        let mut last_write_time = file_obj.metadata().unwrap().modified().unwrap();
        let mut pre_contents = fs::read_to_string(FILE_PATH).expect("Failed to read file");

        let mut interval = tokio::time::interval(Duration::from_secs(2));
        loop {
            let current_write_time = file_obj.metadata().unwrap().modified().unwrap();
            if last_write_time != current_write_time {
                last_write_time = current_write_time;

                let contents = fs::read_to_string(FILE_PATH).expect("Failed to read file");
                if let Some(line) = contents.get(pre_contents.len()..) {
                    // function
                    let split = line.split('%').collect::<Vec<_>>();
                    log_channel
                        .send_message(&ctx, |m| {
                            m.embed(|f| {
                                f.title("BAN-LOG")
                                    .field("IP:", format!("`{}`", split[0]), true)
                                    .field("Player:", format!("`{}`", split[1]), true)
                                    .field("Reason:", format!("`{}`", split[2]), true)
                                    .field("Admin:", format!("`{}`", split[3]), true)
                                    .color(Colour::RED)
                            })
                        })
                        .await
                        .unwrap();
                }

                pre_contents = contents;
            }

            interval.tick().await;
        }
    }
}
