use poise::serenity_prelude as serenity;

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

pub async fn bluebit_check(
    ctx: serenity::prelude::Context,
    msg: poise::serenity_prelude::Message,
) -> bool {
    tokio::time::sleep(std::time::Duration::from_secs(8)).await;
    let nmsg = match msg.channel_id.message(&ctx, msg.id).await {
        Ok(tmsg) => tmsg,
        Err(e) => {
            eprintln!("failed to get message, most likely user deleted it. error: {e}");
            return false;
        }
    };
    let reacts = &nmsg.reactions;
    let correct = [up_arrow!(), down_arrow!(), ques_mark!()];

    let filtered: Vec<_> = reacts
        .iter()
        .filter(|r| correct.contains(&r.reaction_type))
        .collect();

    let mut filter_string = String::new();

    for react in &filtered {
        filter_string = format!("{}{}", filter_string, react.reaction_type);
    }

    if filtered.len() < 3 {
        let mut filter_string = String::new();

        for react in filtered {
            filter_string = format!("{}{}", filter_string, react.reaction_type);
        }

        let content = format!(
            "\
                ## ⚠️ balnkbit detectored\n\
                one or more of the reddit reactions are missing! oh noes!\n\
                reddit reactions: {}\n\
                -# note: this may show up if you're too slow to reddit\
            ",
            filter_string
        );
        nmsg.reply_ping(&ctx, content)
            .await
            .expect("should have been able to send blankbit notifier message");
        return true;
    }

    for (i, r) in correct.iter().enumerate() {
        if *r != filtered[i].reaction_type {
            let content = format!(
                "\
                    ## ⚠️ bleubit detectored!\n\
                    reddit reactions: {}\n\
                    -# note: this may show up if your internet is bad\
                ",
                filter_string
            );
            nmsg.reply_ping(&ctx, content)
                .await
                .expect("should have been able to send bluebit notifier message");
            return true;
        }
    }

    false
}
