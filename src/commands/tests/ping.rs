use crate::{Context, Error};

/// Pong!
#[poise::command(slash_command, prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let response = "Pong!".to_string();
    ctx.say(response).await?;
    Ok(())
}
