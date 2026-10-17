use crate::{Context, Error};
use base64::{Engine as _, engine::general_purpose};

#[derive(poise::ChoiceParameter)]
pub enum Choices {
    #[name = "Decode"]
    Decode,
    #[name = "Encode"]
    Encode,
}

/// Encode/Decode Base64
#[poise::command(slash_command, prefix_command)]
pub async fn base64(
    ctx: Context<'_>,
    #[description = "Encode or decode the input"] choice: Choices,
    #[description = "The input string"] input: String,
) -> Result<(), Error> {
    match choice {
        Choices::Encode => {
            let encoded: String = general_purpose::STANDARD.encode(input);
            ctx.send(poise::CreateReply::default().content(encoded))
                .await?;
        }
        Choices::Decode => {
            let decoded: String = general_purpose::STANDARD
                .decode(input)
                .unwrap()
                .try_into()?;
            ctx.send(poise::CreateReply::default().content(decoded))
                .await?;
        }
    }
    Ok(())
}
