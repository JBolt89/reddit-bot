use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Reacts to a message (owner-only)
#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn react(
    ctx: Context<'_>,
    #[description = "The message ID to react to."] message: String,
    #[description = "The emoji to react with"] reaction: String,
    #[description = "OPTIONAL: The channel the message is in"] channel: Option<serenity::Channel>,
) -> Result<(), Error> {
    if let Some(channel) = channel {
        let msgid: u64 = message.trim().parse()?;
        let msg = channel.id().message(ctx, msgid).await?;
        let reaction = serenity::ReactionType::try_from(reaction)?;
        msg.react(ctx, reaction).await?;
        ctx.send(
            poise::CreateReply::default()
                .content("reacted")
                .ephemeral(true),
        )
        .await?;
    }
    Ok(())
}

// TRC: this is evil as fuck, restricting a command that lets you upvote your own message to only
//      yourself? genuinely evil. just use a normal alt damnit

// GGR: i wasnt gonna use it for that purpose lol
