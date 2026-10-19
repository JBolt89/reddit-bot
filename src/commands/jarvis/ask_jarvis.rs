use crate::commands::jarvis::decider::decide;
use crate::commands::jarvis::jarvis_phrases::JARVIS_PHRASES;
use crate::{Context, Error};

/// Ask Jarvis a question
#[poise::command(slash_command, prefix_command)]
pub async fn ask_jarvis(
    ctx: Context<'_>,
    #[description = "The question to ask Jarvis!"] question: String,
) -> Result<(), Error> {
    let user_id = ctx.author().id;
    let full_string = format!("{question}{user_id}");
    let answer =
        JARVIS_PHRASES[decide(full_string.as_bytes(), JARVIS_PHRASES.len() as u32) as usize];
    ctx.say(format!(
        "Q: {question}\n\
         A: {answer}",
    ))
    .await?;

    Ok(())
}
