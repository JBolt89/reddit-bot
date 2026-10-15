use crate::{Context, Error};

/// Pong!
#[poise::command(slash_command, prefix_command)]
pub async fn ping(
    ctx: Context<'_>,
    // #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    // let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("Pong!");
    ctx.say(response).await?;
    Ok(())
}