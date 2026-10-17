use crate::{Context, Error};

/// Generates a random number in the range min..max (inclusive)
#[poise::command(slash_command, prefix_command)]
pub async fn random_number(
    ctx: Context<'_>,
    #[description = "minimum number"] min: i32,
    #[description = "maximum number"] max: i32,
) -> Result<(), Error> {
    let random = rand::random_range(min..=max);
    ctx.reply(format!("random number from {min} to {max}: {random}"))
        .await?;
    Ok(())
}
