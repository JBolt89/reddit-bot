use crate::{Context, Error};

/// Let our lord Jarvis make a decision for you!
#[poise::command(slash_command, prefix_command)]
pub async fn jarvis_decide(
    ctx: Context<'_>,
    #[description = "The first choice."] choice1: String,
    #[description = "The second choice."] choice2: String,
) -> Result<(), Error> {
    let mut digest = 0_u32;
    let full_string = format!("{}{}", choice1, choice2);

    for c in full_string.chars() {
        digest ^= c as u32;
        digest <<= 5;
    }

    match digest % 2 {
        0 => ctx.say(format!("i think {}", choice1)).await?,
        1 => ctx.say(format!("i think {}", choice2)).await?,
        _ => ctx.say("i think something went wrong").await?,
    };
    Ok(())
}
