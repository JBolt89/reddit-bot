use poise::serenity_prelude::{self as serenity, futures::StreamExt};

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

enum BadbitKind {
    Bluebit,
    Blankbit,
}

struct Badbit {
    kind: BadbitKind,
    reaction_string: String,
    notation_string: String,
}

pub async fn bluebit_check(ctx: serenity::prelude::Context, msg: poise::serenity_prelude::Message) {
    tokio::time::sleep(std::time::Duration::from_secs(8)).await;
    let nmsg = match msg.channel_id.message(&ctx, msg.id).await {
        Ok(tmsg) => tmsg,
        Err(e) => {
            eprintln!("failed to get message, most likely user deleted it. error: {e}");
            return;
        }
    };

    let mut response: Option<serenity::Message> = None;

    let button_id = format!("bitbutton_{}", msg.id);

    let components = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new(&button_id)
            .style(serenity::ButtonStyle::Primary)
            .label("delete reminder"),
    ])];

    match badbit_check(&nmsg).await {
        None => {}
        Some(bit) => match bit.kind {
            BadbitKind::Bluebit => {
                let content = format!(
                    "\
                    # ⚠️ bleubit detectored!\n\
                    ## {}\n\
                    reddit reactions: {}\n\
                    -# note: this may show up if your internet is bad\
                ",
                    bit.notation_string, bit.reaction_string
                );
                response = Some(
                    nmsg.channel_id
                        .send_message(
                            &ctx,
                            serenity::CreateMessage::new()
                                .content(content)
                                .reference_message(&nmsg)
                                .components(components),
                        )
                        .await
                        .expect("should have been able to send bluebit notifier message"),
                );
            }
            BadbitKind::Blankbit => {
                let content = format!(
                    "\
                ### ⚠️ balnkbit detectored\n\
                ## {}\n\
                one or more of the reddit reactions are missing! oh noes!\n\
                reddit reactions: {}\n\
                -# note: this may show up if you're too slow to reddit\
            ",
                    bit.notation_string, bit.reaction_string
                );
                response = Some(
                    nmsg.channel_id
                        .send_message(
                            &ctx,
                            serenity::CreateMessage::new()
                                .content(content)
                                .reference_message(&nmsg)
                                .components(components),
                        )
                        .await
                        .expect("should have been able to send blankbit notifier message"),
                );
            }
        },
    }

    let id_clone = button_id.clone();

    let mut collector = serenity::ComponentInteractionCollector::new(&ctx)
        .channel_id(msg.channel_id)
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == id_clone)
        .stream();

    let nmsg = nmsg
        .channel_id
        .message(&ctx, nmsg.id)
        .await
        .expect("unable to get message");

    while let Some(mci) = collector.next().await {
        let nmsg = nmsg
            .channel_id
            .message(&ctx, nmsg.id)
            .await
            .expect("unable to get message");

        match badbit_check(&nmsg).await {
            Some(bit) => mci
                .create_response(
                    &ctx,
                    serenity::CreateInteractionResponse::Message(
                        serenity::CreateInteractionResponseMessage::new()
                            .content(format!("there is still a bit\n{}", bit.notation_string))
                            .ephemeral(true),
                    ),
                )
                .await
                .expect("couldnt send the reminder that there is still a bit"),
            None => {
                mci.create_response(
                    &ctx,
                    serenity::CreateInteractionResponse::Message(
                        serenity::CreateInteractionResponseMessage::new()
                            .content("deleted reminder")
                            .ephemeral(true),
                    ),
                )
                .await
                .expect("couldnt send a message to say there is no bit");
                match &response {
                    Some(m) => m.delete(&ctx).await.expect("failed to delete"),
                    None => println!("bro what"),
                }
            }
        };
    }
}

async fn badbit_check(msg: &serenity::Message) -> Option<Badbit> {
    let reacts = &msg.reactions;
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
        let mut notation_string = String::new();

        let reaction_types: Vec<serenity::ReactionType> =
            filtered.iter().map(|r| r.reaction_type.clone()).collect();

        for react in correct {
            if !reaction_types.contains(&react) {
                notation_string.push_str(&react.to_string());
            }
        }

        notation_string = notation_string
            .replace(&up_arrow!().to_string(), "u")
            .replace(&down_arrow!().to_string(), "d")
            .replace(&ques_mark!().to_string(), "q");

        return Some(Badbit {
            kind: BadbitKind::Blankbit,
            reaction_string: filter_string,
            notation_string,
        });
    }

    for (i, r) in correct.iter().enumerate() {
        if *r != filtered[i].reaction_type {
            let mut notation_string = String::new();

            let reaction_types: Vec<serenity::ReactionType> =
                filtered.iter().map(|r| r.reaction_type.clone()).collect();

            if reaction_types
                .iter()
                .position(|r| *r == down_arrow!())
                .unwrap()
                < reaction_types
                    .iter()
                    .position(|r| *r == up_arrow!())
                    .unwrap()
            {
                notation_string.push_str("dd");
            }

            if reaction_types
                .iter()
                .position(|r| *r == ques_mark!())
                .unwrap()
                < 2
            {
                notation_string.push_str("qq");
            }

            return Some(Badbit {
                kind: BadbitKind::Bluebit,
                reaction_string: filter_string,
                notation_string,
            });
        }
    }

    None
}
