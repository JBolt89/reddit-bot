use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Reacts to a message (owner-only)
#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn react(
    ctx: Context<'_>,
    #[description = "The message ID to react to."] message: String,
    #[description = "The emoji to react with"] reaction: String,
    #[description = "OPTIONAL: The channel the message is in"] channel: Option<serenity::Channel>
) -> Result<(), Error> {
    
    if channel.is_some() {
        let msgid: u64 = message.trim().parse()?;
        let msg = channel.unwrap().id().message(ctx, msgid).await?;
        let reaction = serenity::ReactionType::try_from(reaction)?;
        msg.react(ctx, reaction).await?;
        ctx.send(poise::CreateReply::default()
            .content("reacted")
            .ephemeral(true)
        ).await?;
    }
    Ok(())
}