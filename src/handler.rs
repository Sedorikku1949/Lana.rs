pub(in crate::handler) mod commands {
    use std::process;

    use serenity::client::bridge::gateway::ShardId;
    use serenity::model::channel::Message;
    use serenity::model::prelude::Activity;
    use serenity::prelude::Context;

    use crate::executor::CommandArgs;
    use crate::{ decrypt, ShardManagerContainer };
    use crate::embeder;
    use crate::vars::{DEV_ID, CONFIG};

    pub(in crate::handler) async fn ping(ctx: &Context, message: &Message, _opt: &CommandArgs){
        let data = ctx.data.read().await;

        let shard_manager = match data.get::<ShardManagerContainer>() {
            Some(v) => v,
            None => { let _ = message.reply(ctx, "There was a problem getting the shard manager").await; return; },
        };

        let manager = shard_manager.lock().await;
        let runners = manager.runners.lock().await;

        let runner = match runners.get(&ShardId(ctx.shard_id)) {
            Some(runner) => runner,
            None => { let _ = message.reply(ctx, "No shard found").await; return; },
        };

        let _ = match runner.latency {
            Some(latency) => message.reply(ctx, &format!("> 🏓 **Ma latence est de** `{:?} ms` !", latency.as_millis())).await,
            _ => message.reply(ctx, &format!("> 🦀 ** ** **I have a Rusty error on here.**\nJe n'arrive pas à obtenir ma latence.")).await,
        };
        return;
    }
    
    pub(in crate::handler) async fn test(ctx: &Context, message: &Message, opt: &CommandArgs){
        if let Err(why) = message.channel_id.say(&ctx.http, format!("arguments: {}", opt.args.join(" "))).await { println!("Error sending message: {:?}", why); }
    }
    
    pub(in crate::handler) async fn exit(ctx: &Context, message: &Message, _opt: &CommandArgs){
        let dev_id_list = DEV_ID.map(|id| decrypt(id, true).unwrap()).to_vec();
        if !dev_id_list.contains(&message.author.id.to_string()) {
            let _ = message.reply(ctx, "> ⛔ ** ** **Tu ne possède pas les permissions requises.**").await;
            return;
        } else {
            // dev
            let _ = message.reply(ctx, "> 🦀 ** ** **Je retourne dans ma caverne...**").await;
            process::exit(0);
        }
    }
    
    pub(in crate::handler) async fn maintenance(ctx: &Context, message: &Message, _opt: &CommandArgs){
        let dev_id_list = DEV_ID.map(|id| decrypt(id, true).unwrap()).to_vec();
        if !dev_id_list.contains(&message.author.id.to_string()){
            let _ = message.reply(ctx, "> ⛔ ** ** **Tu ne possède pas les permissions requises.**").await;
            return;
        } else {
            unsafe {
                if CONFIG.maintenance > 0 {
                    // maintenance on, disabled it
                    CONFIG.maintenance = 0;
                    let _ = CONFIG.save();
                    let __ = message.reply(ctx, "> 🔓 ** ** **Maintenance désactivée !**").await;
                    CONFIG.status = CONFIG.default_status.clone();
                    ctx.set_activity(Activity::listening(CONFIG.status)).await;
                    return;
                }
                // maintenance off, enable it
                CONFIG.maintenance = 1;
                let _ = CONFIG.save();
                let __ = message.reply(ctx, "> 🔒 ** ** **Maintenance activée !**").await;
                CONFIG.status = "🔒 Maintenance in progress";
                ctx.set_activity(Activity::listening(CONFIG.status)).await;
                return;
            }
        }
    }

    pub(in crate::handler) async fn lua_test(ctx: &Context, message: &Message, _opt: &CommandArgs){
        let dev_id_list = DEV_ID.map(|id| decrypt(id, true).unwrap()).to_vec();
        if !dev_id_list.contains(&message.author.id.to_string()){
            let _ = message.reply(ctx, "> ⛔ ** ** **Tu ne possède pas les permissions requises.**").await;
            return;
        } else {
            unsafe {
                let lua_res = embeder::processor::cpu::test();
                match lua_res {
                    Ok(result) => {
                        let _ = message.reply(ctx, format!("> ✅ ** ** **Succès !**\n```lua\n{}\n```", result)).await;
                        return;
                    },
                    _ => {
                        let __ = message.reply(ctx, "> 💥 ** ** **Une erreur est survenue en chargeant le render.**").await;
                        return;
                    }
                }
            }
        }
    }
}

pub mod executor {
    use chrono::Utc;
    use serenity::model::channel::Message;
    use serenity::prelude::Context;
    use crate::handler::commands::{exit, ping, test};
    use crate::vars::PREFIX;

    use super::commands::{maintenance, lua_test};

    fn format_args(content: &String) -> Vec<String> { return content.split(char::is_whitespace).map(|s| String::from(s.trim())).collect() }

    pub(in crate::handler) struct CommandArgs {
        pub(in crate::handler) args: Vec<String>,
        pub(in crate::handler) command_name: String
    }

    impl CommandArgs {
        fn new(args: &Vec<String>) -> CommandArgs {
            let options = &args[1..];
            CommandArgs { args: Vec::from(options.to_owned()), command_name: args.get(0).unwrap().to_string().replace("&", "") }
        }
    }

    fn after(_ctx: &Context, message: &Message, opt: &CommandArgs) {
        println!(
            "[\x1b[34mCommand\x1b[0m] command \x1b[35m{command_name}\x1b[0m has been executed by \x1b[34m{username}\x1b[0m (\x1b[34m{user_id}\x1b[0m) at \x1b[32m{date}\x1b[0m",
            command_name = opt.command_name,
            username = message.author.name,
            user_id = message.author.id,
            date = Utc::now().format("%d/%m/%Y %H:%M")
        )
    }

    pub async fn received(ctx: &Context, message: &Message){
        let msg_content = &message.content;
        if msg_content.len() < 1 || msg_content.chars().take(1).last().unwrap() != PREFIX {}
        else {
            // before
            let args: Vec<String> = format_args(&msg_content);
            let command = args.get(0).unwrap().as_str();
            let opt: CommandArgs = CommandArgs::new(&args);

            // execute command
            match command {
                "&ping" => {
                    ping(&ctx, &message, &opt).await;
                    after(&ctx, &message, &opt)
                },
                "&test" => {
                    test(&ctx, &message, &opt).await;
                    after(&ctx, &message, &opt)
                },
                "&exit" | "&stop" => {
                    exit(&ctx, &message, &opt).await;
                    after(&ctx, &message, &opt)
                },
                "&maintenance" => {
                    maintenance(&ctx, &message, &opt).await;
                    after(&ctx, &message, &opt)
                },
                "&lua" => {
                    lua_test(&ctx, &message, &opt).await;
                    after(&ctx, &message, &opt)
                },
                _ => {}
            }
        }
    }
}
