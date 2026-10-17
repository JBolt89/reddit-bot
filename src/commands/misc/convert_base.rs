use crate::{Context, Error};
use poise::{ChoiceParameter};

#[derive(poise::ChoiceParameter)]
pub enum Bases {
    #[name = "Decimal (Base 10)"]
    Decimal,
    #[name = "Binary (Base 2)"]
    Binary,
    #[name = "Hexadecimal (Base 16)"]
    Hex,
}

/// Converts a number to a different base.
#[poise::command(slash_command, prefix_command, owners_only)]
pub async fn convert_base(
    ctx: Context<'_>,
    #[description = "The number to convert."] number: String,
    #[description = "The base to convert from."] convert_from: Bases,
    #[description = "The base to convert to."] convert_to: Bases,
    #[description = "Whether the message is ephemeral or not"] ephemeral: Option<bool>
) -> Result<(), Error> {
    let converted = match convert_from {
        Bases::Decimal => i64::from_str_radix(&number, 10).unwrap(),
        Bases::Binary => i64::from_str_radix(&number, 2).unwrap(),
        Bases::Hex => i64::from_str_radix(&number, 16).unwrap()
    };

    match convert_to {
        Bases::Decimal => {
            ctx.send(poise::CreateReply::default()
                .content(format!("{} ({}) in {} is {}", number, convert_from.name(), convert_to.name(), converted))
                .ephemeral(ephemeral.unwrap())).await?;
        },
        Bases::Binary => {
            ctx.send(poise::CreateReply::default()
                .content(format!("{} ({}) in {} is {:#b}", number, convert_from.name(), convert_to.name(), converted))
                .ephemeral(ephemeral.unwrap())).await?;
        },
        Bases::Hex => {
            ctx.send(poise::CreateReply::default()
                .content(format!("{} ({}) in {} is {:#018x} (leading 0s as it's i64)", number, convert_from.name(), convert_to.name(), converted))
                .ephemeral(ephemeral.unwrap())).await?;
        },
    }
    Ok(())
}