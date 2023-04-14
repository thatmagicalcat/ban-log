mod handler;
mod watcher;

pub use handler::Handler;
pub use watcher::watch;

pub const CHANNEL_ID: serenity::model::prelude::ChannelId = serenity::model::prelude::ChannelId(978655035674218497);
pub const GUILD_ID: serenity::model::prelude::GuildId = serenity::model::prelude::GuildId(978654617938296932);
pub const FILE_PATH: &str = "/home/ubuntu/sd/main/miscmod_bans.dat";
