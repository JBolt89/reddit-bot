use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Pong!
#[poise::command(slash_command, prefix_command)]
pub async fn react(
    ctx: Context<'_>,
    #[description = "The message ID to react to."] message: serenity::Message,
) -> Result<(), Error> {
    let response = format!("Pong!");
    ctx.say(response).await?;
    Ok(())
}