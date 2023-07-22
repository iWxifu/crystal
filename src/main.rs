mod texts;

use std::env;
use dotenv::dotenv;
use regex::Regex;

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::user::OnlineStatus;
use serenity::model::channel::Message;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::prelude::Activity;
use serenity::prelude::*;

struct Bot {}

#[async_trait]
impl EventHandler for Bot {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                _ => "not done :(".to_string(),
            };

            if let Err(fuck) = command.create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|message| message.content(content))
            }).await
            {
                println!("Can't respond to Slash Command: {}", fuck);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} connected!", ready.user.name);
        ctx.set_presence(None, OnlineStatus::DoNotDisturb).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {

        if msg.author.bot {
            return;
        }

        if let Some(matched) = match_regex(&msg.content) {
            if let Err(_) = msg.channel_id.say(&ctx.http, texts::get_toxic()).await {
                println!("error sending message");
            } else {
                return;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("BOT_TOKEN").expect("Expected a Discord Token");

    let bot = Bot {};

    let intents = GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .event_handler(bot)
        .await
        .expect("Error creating client");

    if let Err(fml) = client.start().await {
        println!("Client Error: {:#?}", fml);
    }
}

fn match_regex(input: &str) -> Option<String> {
    let regex_patterns = vec![
        r"\b(uwu)\b",
        r"\b(bitch)\b",
        r"^fuck you crystal$",
        r"^[Cc][Rr][Yy][Ss][Tt][Aa][Ll]$",
        r"\b[rR][oO][bB][lL][oO][xX]\b",
        r"\b(anarchy server)\b",
        r"\^crystal tell me a secret$",
        r"\b(stupid bot)\b",
        r"^crystal help$",
        r"\?$",
        r"^(?i)well, actually\b",
        r"^/bump$",
        r"\b(join vc)\b",
        r"^ruby join vc$",
        r"\b(vc)\b",
        r"^sorry crystal$",
        r"\b(oof)\b",
        r"^hi crystal$"
    ];

    for pattern in regex_patterns {
        let re = Regex::new(pattern).unwrap();
        if let Some(mat) = re.find(input) {
            println!("{}", mat.as_str().to_string());
            return Some(mat.as_str().to_string().to_ascii_lowercase());
        }
    }

    None
}