use crate::{Context, Error};

/// Evaluate math!
#[poise::command(slash_command, prefix_command)]
pub async fn math(
    ctx: Context<'_>,
    #[description = "the expression to evaluate"] expression: String,
    #[description = "OPTIONAL (default false): if the response is ephemeral (only shown to you)"]
    ephemeral: Option<bool>,
) -> Result<(), Error> {
    match ephemeral {
        Some(b) => {
            let result: f64 = match meval::eval_str(&expression) {
                Ok(i) => i,
                Err(e) => {
                    ctx.send(
                        poise::CreateReply::new()
                            .content(format!("an error occurred evaluating the expression: {e}"))
                            .ephemeral(b),
                    )
                    .await?;
                    return Ok(());
                }
            };

            ctx.send(
                poise::CreateReply::new()
                    .content(format!("{} = {}", expression, result))
                    .ephemeral(b),
            )
            .await?;
        }
        None => {
            let result = match meval::eval_str(&expression) {
                Ok(i) => i,
                Err(e) => {
                    ctx.send(
                        poise::CreateReply::new()
                            .content(format!("an error occurred evaluating the expression: {e}"))
                            .ephemeral(false),
                    )
                    .await?;
                    return Ok(());
                }
            };

            ctx.say(format!("{} = {}", expression, result)).await?;
        }
    };

    Ok(())
}
