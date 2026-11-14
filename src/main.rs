use std::{collections::HashMap, sync::Mutex};

use poise::serenity_prelude as serenity;
// GGR: for some reason i couldnt use futures::StreamExt without doing this
// tried searching for a way but i couldnt find one so its this now lmao

use crate::cablab_variants::CABLAB_VARIANTS;

mod commands {
    pub mod tests {
        pub mod ping;
    }
    pub mod owner_tools {
        pub mod dm_user;
        pub mod react;
    }
    pub mod misc {
        pub mod base64_command;
        pub mod convert_base;
        pub mod math;
        pub mod random_number;
        pub mod say;
    }
    pub mod jarvis {
        pub mod ask_jarvis;
        mod decider;
        pub mod jarvis_decide;
        pub mod jarvis_phrases;
    }
    pub mod fun {
        pub mod gamble;
    }
    pub mod admin {
        pub mod timeout_role;
    }
}

mod event_functions {
    pub mod anti_cablab;
    pub mod bluebit_check;
    pub mod nullscape_check;
}

mod cablab_variants;
mod ns_references;

#[cfg(test)]
mod tests;

macro_rules! def_reaction {
    ($name:ident!() = $emoji:literal) => {
        macro_rules! $name {
            () => {
                serenity::ReactionType::Unicode($emoji.to_string())
            };
        }
    };
}

def_reaction!(up_arrow!() = "⬆️");
def_reaction!(down_arrow!() = "⬇️");
def_reaction!(ques_mark!() = "❓");
def_reaction!(pickle!() = "🥒");

const RTOC_GUILD: u64 = 1530571157651329054;
const STARBOARD_BOT: u64 = 655390915325591629;
const TBC_ALT_ROLE: u64 = 1544982028515282964;
const PIH_CHANNEL: u64 = 1571059108835106866;

/// User data, which is stored and accessible in all command invocations
struct Data {
    money: Mutex<HashMap<u64, i64>>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string("token.txt").expect("token.txt file should exist");
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::tests::ping::ping(),
                commands::owner_tools::react::react(),
                commands::owner_tools::dm_user::dm_user(),
                commands::misc::random_number::random_number(),
                commands::misc::convert_base::convert_base(),
                commands::misc::base64_command::base64(),
                commands::misc::say::say(),
                commands::misc::math::math(),
                commands::jarvis::ask_jarvis::ask_jarvis(),
                commands::jarvis::jarvis_decide::jarvis_decide(),
                commands::fun::gamble::gamble(),
                commands::fun::gamble::gamble_seeded(),
                commands::admin::timeout_role::timeout_role(),
            ],
            event_handler: |framework, event| Box::pin(event_handler(framework, event)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    money: Mutex::new(HashMap::new()),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

fn is_pickle_message(message: &str) -> bool {
    let mut p = false;
    let mut i = false;
    let mut c = false;
    let mut k = false;
    let mut l = false;
    for char in message.chars() {
        match char {
            'p' => p = true,
            'i' | '1' if p => i = true,
            'c' if i => c = true,
            'k' if c => k = true,
            'l' if k => l = true,
            'e' | '3' if l => return true,
            '🥒' => return true,
            ch if ch.is_alphabetic() => [p, i, c, k, l] = [false; 5],
            _ => {}
        }
    }
    false
}

async fn event_handler(
    framework: poise::FrameworkContext<'_, Data, Error>,
    event: &serenity::FullEvent,
) -> Result<(), Error> {
    let ctx = framework.serenity_context;

    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.name);
            ctx.set_activity(Some(serenity::ActivityData::watching(
                "your redditification",
            )));
        }
        serenity::FullEvent::Message { new_message } => {
            if new_message.content.to_lowercase() == "dumbass" {
                new_message.reply(ctx, "no YOU dumbass").await?;
            }

            if is_pickle_message(&new_message.content) {
                new_message.react(ctx, pickle!()).await?;
            }

            if new_message.author.bot {
                if new_message.author.id == STARBOARD_BOT {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    new_message.react(ctx, ques_mark!()).await?;
                } else {
                    new_message.react(ctx, up_arrow!()).await?;
                    new_message.react(ctx, down_arrow!()).await?;
                    new_message.react(ctx, ques_mark!()).await?;
                }
            } else {
                let ctx_clone = ctx.clone();
                let msg_clone = new_message.clone();
                tokio::spawn(event_functions::bluebit_check::bluebit_check(
                    ctx_clone, msg_clone,
                ));
                let ctx_clone = ctx.clone();
                let msg_clone = new_message.clone();
                tokio::spawn(event_functions::nullscape_check::nullscape_check(
                    ctx_clone, msg_clone,
                ));
                let ctx_clone = ctx.clone();
                let msg_clone = new_message.clone();
                tokio::spawn(event_functions::anti_cablab::anti_cablab(
                    ctx_clone, msg_clone,
                ));
            }
        }
        _ => {}
    }
    Ok(())
}
