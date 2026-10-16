use poise::serenity_prelude::{self as serenity, ReactionType::{Unicode}};

mod commands {
    pub mod tests {
        pub mod ping;
    }
    pub mod owner_tools {
        pub mod react;
    }
    pub mod misc {
        pub mod convert_base;
    }
}

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string("token.txt").unwrap();
    let intents = serenity::GatewayIntents::all();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::tests::ping::ping(), commands::owner_tools::react::react(), commands::misc::convert_base::convert_base()],
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
            let mut alphabetic = new_message.content.clone();
            alphabetic.retain(|c| c.is_alphanumeric());

            if alphabetic
                .to_lowercase()
                .replace("1", "i")
                .contains("pickle") {
                    new_message.react(ctx, Unicode(("🥒").to_string())).await?;
                }

            if new_message.author.bot {
                if new_message.author.id == 655390915325591629 {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    new_message.react(ctx, Unicode(("❓").to_string())).await?;
                }
                else {
                    new_message.react(ctx, Unicode(("⬆️").to_string())).await?;
                    new_message.react(ctx, Unicode(("⬇️").to_string())).await?;
                    new_message.react(ctx, Unicode(("❓").to_string())).await?;
                }
            }
            else {
                let ctx_clone = ctx.clone();
                let msg_clone = new_message.clone();
                tokio::spawn(async move {bluebit_check(ctx_clone, msg_clone).await});
                let ctx_clone = ctx.clone();
                let msg_clone = new_message.clone();
                tokio::spawn(nullscape_check(ctx_clone, msg_clone));
            }
        }
        _ => {}
    }
    Ok(())
}

async fn bluebit_check(ctx: serenity::prelude::Context, msg: poise::serenity_prelude::Message) -> bool {
    tokio::time::sleep(std::time::Duration::from_secs(8)).await;
    let nmsg = msg.channel_id.message(&ctx, msg.id).await.unwrap();
    let reacts = &nmsg.reactions;
    let correct = [serenity::ReactionType::Unicode("⬆️".to_string()), serenity::ReactionType::Unicode("⬇️".to_string()), serenity::ReactionType::Unicode('❓'.to_string())];

    let filtered: Vec<_> = reacts
      .into_iter()
      .filter_map(|r| correct.contains(&r.reaction_type).then_some(&r.reaction_type))
      .collect();

    if filtered.len() < 3 {
        nmsg.reply(&ctx, "## ⚠️ balnkbit detectored\none or more of the reddit reactions are missing! oh noes!\n-# note: this may show up if you're too slow to reddit").await.expect("wtf {e}");
        return true;
    }

    for (i, r) in correct.iter().enumerate() {
        if *r != *filtered[i] {
            let content = format!("## ⚠️ bluebit detected!\nreddit reactions: {} {} {}", filtered[0], filtered[1], filtered[2]);
            nmsg.reply(&ctx, content).await.expect("huh");
            return true;
        }
    }

    return false;
}

async fn nullscape_check(ctx: serenity::prelude::Context, msg: poise::serenity_prelude::Message) {
    let references = ["null", "scape", "mart", "baby", "celest", "flame", "bell", "symphony", "husk", "insurmountable", "abyss", "decay", "spring", "missile", "icbm", "flesh", "operator", "guardian", "tele", "greed", "purg", "nothing", "colon", "cadence", "sigil", "scrap", "random", "???", "ponder", "nil", "catalyst", "curse", "enemy", "upgrade", "greater", "dive", "charge", "spirit", "grapple", "glide", "wanted", "prisoner", "altar", "matrix", "swiftness", "paycheck", "medal", "esp", "belt", "wing", "trip", "shark", "tail", "gift", "blossom", "bloom", "fall", "boom", "panic", "shield", "shoe", "drown", "business", "run", "tile", "collapse", "gold", "mine", "chaos", "beacon", "patch", "chance", "protect", "pure", "razor", "sub", "space", "i guess bro", "ig bro", "votekick", "its time", "it's time", "operate", "bitter", "cease", "silence", "bound", "echo", "adrenaline", "pass", "mirage", "oblivion", "jackpot", "fragile", "trauma", "lap", "camo", "stairs", "void"];
    if references.iter().any(|s| msg.content.contains(s)) {
        msg.react(ctx, serenity::ReactionType::try_from("<:SubspaceTripmine:1560623715861926030>").unwrap()).await.expect("thing");
    }
}