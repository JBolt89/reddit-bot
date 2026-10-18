use crate::{Context, Error, commands::jarvis::decider::decide};

/// Let our lord Jarvis make a decision for you!
#[poise::command(slash_command, prefix_command)]
pub async fn jarvis_decide(
    ctx: Context<'_>,
    #[description = "The first choice."] choice1: String,
    #[description = "The second choice."] choice2: String,
) -> Result<(), Error> {
    let full_string = format!("{}{}", choice1, choice2);
    ctx.say(format!(
        "i think {}",
        [choice1, choice2][decide(full_string.as_bytes(), 2) as usize]
    ))
    .await?;
    Ok(())
}
