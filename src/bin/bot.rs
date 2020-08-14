use dotenv::dotenv;
use log::{error, info};
use std::env;

use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use serenity::model::prelude::*;
use serenity::Client;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    Args,
    macros::{command, group},
};

#[command]
async fn doc(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    info!("message (args): {:?}", args.message());
    let search_results = djangodocbot::search_documentation(args.message()).await;
    let message = search_results
        .iter()
        .take(3)
        .fold(MessageBuilder::new().push("\n"), |m, result| {
            m.push_bold_line(&result.title);
            m.push_line(&result.url);
            m.push("\n");
            m
        })
    .build();
    match msg.channel_id.send_message( &ctx, |m| { 
        m.embed(|e| {
            e.description(message);
            e
        });
        m
    }).await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("couldn't send message: {:?}", e);
            Ok(())
        }
    }
}

#[group]
#[commands(doc)]
struct Commands;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"))
        .framework(
            StandardFramework::new()
                .configure(|c| c.prefix(&env::var("DISCORD_PREFIX").unwrap_or("!".to_string())))
                .group(&COMMANDS_GROUP),
            )
        .await.expect("Error creating client");
    
    if let Err(why) = client.start().await {
        error!("An error occurred while running the client: {:?}", why);
    }
}
