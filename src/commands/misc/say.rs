use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity};

/// Makes the bot send a message!
#[poise::command(slash_command, prefix_command)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "The text for the bot to say."] text: String,
    #[description = "OPTIONAL: The channel to say it in."] channel: Option<serenity::Channel>,
) -> Result<(), Error> {
    let channel_target: serenity::ChannelId;

    match channel {
        Some(channel) => channel_target = channel.id(),
        None => channel_target = ctx.channel_id(),
    }

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
