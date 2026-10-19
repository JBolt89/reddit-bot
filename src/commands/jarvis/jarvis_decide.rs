use crate::{Context, Error, commands::jarvis::decider::decide};

/// Let our lord Jarvis make a decision for you!
#[poise::command(slash_command, prefix_command)]
pub async fn jarvis_decide(
    ctx: Context<'_>,
    #[description = "The first choice."] choice1: String,
    #[description = "The second choice."] choice2: String,
) -> Result<(), Error> {
    let user_id = ctx.author().id;
    let full_string = format!("{}{}{user_id}", choice1, choice2);
    let decision = [&choice1, &choice2][decide(full_string.as_bytes(), 2) as usize];
    ctx.say(format!(
        "Q: should I {choice1} or {choice2}?\n\
         A: i think you should {decision}",
    ))
    .await?;
    Ok(())
}
