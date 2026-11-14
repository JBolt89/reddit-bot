use crate::{CABLAB_VARIANTS, PIH_CHANNEL, RTOC_GUILD, TBC_ALT_ROLE};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::futures::StreamExt;

macro_rules! warning {
    () => {
        serenity::ReactionType::Unicode("⚠️".to_string())
    };
}

pub async fn anti_cablab(ctx: serenity::prelude::Context, msg: poise::serenity_prelude::Message) {
    let contains_cablab = CABLAB_VARIANTS
        .iter()
        .any(|s| msg.content.to_lowercase().contains(s));

    if contains_cablab {
        msg.react(&ctx, warning!()).await.expect("couldnt react :(");
        serenity::ChannelId::from(PIH_CHANNEL)
            .send_message(
                &ctx,
                serenity::CreateMessage::new()
                    .content(format!(
                        "\
                        ## c\\*blab detectored!\n\
                        channel: <#{}>\n\
                        message link: {}\n\
                        time sent: <t:{}:S>\n\
                        message author: {} (<@{}>)\n\
                    ",
                        msg.channel_id,
                        msg.link(),
                        msg.timestamp.unix_timestamp(),
                        msg.author.name,
                        msg.author.id
                    ))
                    .add_file(serenity::CreateAttachment::bytes(
                        msg.content.into_bytes(),
                        "content.txt",
                    ))
                    .allowed_mentions(serenity::CreateAllowedMentions::new()),
            )
            .await
            .expect("failed to log c*blab!");
    }

    if msg
        .author
        .has_role(&ctx, RTOC_GUILD, TBC_ALT_ROLE)
        .await
        .expect("error occurred checking roles")
        && contains_cablab
    {
        let mut members = serenity::GuildId::from(RTOC_GUILD)
            .members_iter(&ctx.http)
            .boxed();

        while let Some(member) = members.next().await {
            match member {
                Ok(mut member) => {
                    if !member
                        .user
                        .has_role(&ctx, RTOC_GUILD, TBC_ALT_ROLE)
                        .await
                        .expect("error checking roles")
                    {
                        continue;
                    }

                    let timestamp = serenity::Timestamp::from_unix_timestamp(
                        serenity::Timestamp::now().unix_timestamp() + 1800,
                    );

                    match timestamp {
                        Err(error) => eprintln!("error occurred calculating timestamp: {error}"),
                        Ok(timestamp) => {
                            member
                                .disable_communication_until_datetime(&ctx, timestamp)
                                .await
                                .expect("error occurred timing out user");
                        }
                    }
                }
                Err(error) => eprintln!("error occurred reading members: {error}"),
            }
        }
    }
}
