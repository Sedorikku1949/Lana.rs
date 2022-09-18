#[macro_use] extern crate magic_crypt;

mod vars;
mod security;
mod handler;
mod embeder;

use std::fs::OpenOptions;
use std::io::Read;
use std::sync::Arc;
use dotenvy::dotenv;

use crate::security::crypto::{ decrypt };
use crate::vars::TOKEN;
use crate::handler::executor;
use crate::vars::CONFIG;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::*;

use serenity::model::channel::{ Message};
use serenity::model::gateway::{ GatewayIntents, Ready };
use serenity::model::prelude::Activity;


struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        executor::received(&ctx, &msg).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("[\x1b[34mClient\x1b[0m] \x1b[35m{}\x1b[0m \x1b[34mis connected!\x1b[0m ", ready.user.name);

        // set activity
        unsafe {
            if CONFIG.maintenance > 0 { CONFIG.status = "🔒 Maintenance in progress..." }
            ctx.set_activity(Activity::listening(&CONFIG.status)).await;
        };
        return;
    }
}


#[tokio::main]
async fn main() {
    dotenv().ok();

    // load config file
    {
        let mut config_file = OpenOptions::new().read(true).write(true).create(true).open("config.lmay").expect("Cannot load config file !! Be sure to create \"config.lmay\"");
        let mut config_data: String = String::new();
        config_file.read_to_string(&mut config_data).expect("TODO: panic message");
        for line in config_data.lines() {
            let d: Vec<&str> = line.trim().splitn(2, "=").collect();
            match d.get(0) {
                Some(&"maintenance") => unsafe {
                    if let Some(v) = d.get(1) {
                        if v == &"1" {
                            CONFIG.maintenance = 1;
                            println!("[\x1b[34mStorage\x1b[0m] \x1b[33mMaintenance was enabled on the last session, enable it again\x1b[0m")
                        } else {
                            CONFIG.maintenance = 0;
                            println!("[\x1b[34mStorage\x1b[0m] \x1b[32mMaintenance disabled on the last session, keep this state\x1b[0m")
                        }
                    }
                },
                _ => unsafe {
                    let k: String = d.get(0).unwrap().to_string();
                    let v: String = d.get(1).unwrap().to_string();
                    println!("[\x1b[34mStorage\x1b[0m] \x1b[32menv variable\x1b[0m {key} \x1b[32mstored with value\x1b[0m {value}", key = k, value = v);
                    CONFIG.edit(&k, &v, false);
                }
            }
        }
    }


    let token = decrypt(&TOKEN, true).expect("Cannot resolve token !!!");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    // init shards && shard config
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start_shard(0, 1).await {
        println!("Client error: {:?}", why);
    }
}