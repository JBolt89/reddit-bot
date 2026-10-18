use crate::{Context, Error, RTOC_GUILD, TBC_ALT_ROLE};
use poise::serenity_prelude as serenity;

/// Makes the bot send a message!
#[poise::command(slash_command, prefix_command)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "The text for the bot to say."] text: String,
    #[description = "OPTIONAL: The channel to say it in."] channel: Option<serenity::Channel>,
) -> Result<(), Error> {
    if ctx.author().has_role(ctx, RTOC_GUILD, TBC_ALT_ROLE).await? {
        ctx.send(
            poise::CreateReply::default()
                .content("# __**no**__")
                .ephemeral(false),
        )
        .await?;
        return Ok(());
    }

    let channel_target: serenity::ChannelId = match channel {
        Some(channel) => channel.id(),
        None => ctx.channel_id(),
    };

    channel_target
        .send_message(ctx.http(), serenity::CreateMessage::new().content(text))
        .await?;
    ctx.send(
        poise::CreateReply::default()
            .content("sent the message")
            .ephemeral(true),
    )
    .await?;

    Ok(())
}
