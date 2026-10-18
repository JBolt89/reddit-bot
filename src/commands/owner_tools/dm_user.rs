use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// DMs a user. OWNER ONLY!
#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn dm_user(
    ctx: Context<'_>,
    #[description = "The user to DM"] user: serenity::User,
    #[description = "The content to DM the user"] content: String,
) -> Result<(), Error> {
    user.direct_message(ctx, serenity::CreateMessage::new().content(content))
        .await?;

    Ok(())
}
