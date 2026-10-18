use crate::{Context, Error};
use crate::commands::jarvis::jarvis_phrases::JARVIS_PHRASES;

/// Ask Jarvis a question
#[poise::command(slash_command, prefix_command)]
pub async fn ask_jarvis(
    ctx: Context<'_>,
    #[description = "The question to ask Jarvis!"] question: String
) -> Result<(), Error> {
    
    let mut digest = 0_u32;

    for c in question.chars() {
        digest ^= c as u32;
        digest <<= 5;
    }

    ctx.say(format!("Q: {}\nA: {}", question, JARVIS_PHRASES[digest as usize % JARVIS_PHRASES.len()])).await?;

    Ok(())
}