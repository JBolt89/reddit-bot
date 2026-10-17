use poise::serenity_prelude as serenity;

use crate::ns_references::NULLSCAPE_REFERENCES;

mod commands {
    pub mod tests {
        pub mod ping;
    }
    pub mod owner_tools {
        pub mod react;
    }
    pub mod misc {
        pub mod base64_command;
        pub mod convert_base;
        pub mod random_number;
        pub mod say;
    }
}
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

/// User data, which is stored and accessible in all command invocations
struct Data {}

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
                commands::misc::random_number::random_number(),
                commands::misc::convert_base::convert_base(),
                commands::misc::base64_command::base64(),
                commands::misc::say::say(),
            ],
            event_handler: |framework, event| Box::pin(event_handler(framework, event)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
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
                tokio::spawn(async move { bluebit_check(ctx_clone, msg_clone).await });
                let ctx_clone = ctx.clone();
                let msg_clone = new_message.clone();
                tokio::spawn(nullscape_check(ctx_clone, msg_clone));
            }
        }
        _ => {}
    }
    Ok(())
}

async fn bluebit_check(
    ctx: serenity::prelude::Context,
    msg: poise::serenity_prelude::Message,
) -> bool {
    tokio::time::sleep(std::time::Duration::from_secs(8)).await;
    let nmsg = msg.channel_id.message(&ctx, msg.id).await.unwrap();
    let reacts = &nmsg.reactions;
    let correct = [up_arrow!(), down_arrow!(), ques_mark!()];

    let filtered: Vec<_> = reacts
        .iter()
        .filter_map(|r| {
            correct
                .contains(&r.reaction_type)
                .then_some(&r.reaction_type)
        })
        .collect();

    if filtered.len() < 3 {
        nmsg.reply(
            &ctx,
            "\
                ## ⚠️ balnkbit detectored\n\
                one or more of the reddit reactions are missing! oh noes!\n\
                -# note: this may show up if you're too slow to reddit\
            ",
        )
        .await
        .expect("should have been able to send blankbit notifier message");
        return true;
    }

    for (i, r) in correct.iter().enumerate() {
        if *r != *filtered[i] {
            let content = format!(
                "\
                    ## ⚠️ bleubit detectored!\n\
                    reddit reactions: {}{}{}\n\
                    -# note: this may show up if your internet is bad\
                ",
                filtered[0], filtered[1], filtered[2]
            );
            nmsg.reply(&ctx, content)
                .await
                .expect("should have been able to send bluebit notifier message");
            return true;
        }
    }

    false
}

async fn nullscape_check(ctx: serenity::prelude::Context, msg: poise::serenity_prelude::Message) {
    if NULLSCAPE_REFERENCES.iter().any(|s| msg.content.contains(s)) {
        msg.react(
            ctx,
            serenity::ReactionType::try_from("<:SubspaceTripmine:1560623715861926030>").unwrap(),
        )
        .await
        .expect("should have been able to react subspace tripmine");
    }
}
