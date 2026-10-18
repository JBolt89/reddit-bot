use crate::{Context, Error};
use crate::commands::jarvis::jarvis_phrases::JARVIS_PHRASES;

/// Ask Jarvis a question
#[poise::command(slash_command, prefix_command)]
pub async fn ask_jarvis(
    ctx: Context<'_>,
    #[description = "The question to ask Jarvis!"] question: String
) -> Result<(), Error> {
    let phrase = JARVIS_PHRASES[rand::random_range(0..JARVIS_PHRASES.len())];

    ctx.say(format!("Q: {}\nA: {}", question, phrase)).await?;

    Ok(())
}