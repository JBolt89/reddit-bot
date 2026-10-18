use crate::commands::jarvis::decider::decide;
use crate::commands::jarvis::jarvis_phrases::JARVIS_PHRASES;
use crate::{Context, Error};

/// Ask Jarvis a question
#[poise::command(slash_command, prefix_command)]
pub async fn ask_jarvis(
    ctx: Context<'_>,
    #[description = "The question to ask Jarvis!"] question: String,
) -> Result<(), Error> {
    ctx.say(format!(
        "Q: {}\nA: {}",
        question,
        JARVIS_PHRASES[decide(question.as_bytes(), JARVIS_PHRASES.len() as u32) as usize]
    ))
    .await?;

    Ok(())
}

