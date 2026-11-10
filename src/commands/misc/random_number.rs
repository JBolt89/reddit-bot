use rand::{RngExt, SeedableRng};

use crate::{Context, Error};

/// Generates a random number in the range min..max (inclusive)
#[poise::command(slash_command, prefix_command)]
pub async fn random_number(
    ctx: Context<'_>,
    #[description = "minimum number"] min: i32,
    #[description = "maximum number"] max: i32,
    #[description = "OPTIONAL: the seed to give the RNG"] seed: Option<u64>,
) -> Result<(), Error> {
    let mut response = format!("random number from {min} to {max}: ");

    match seed {
        Some(s) => {
            let mut rng = rand::rngs::StdRng::seed_from_u64(s);
            let number = rng.random_range(min..=max);
            response.push_str(format!("{number}\n-# using seed {s}").as_str());
        }
        None => {
            let number = rand::random_range(min..=max);
            response.push_str(format!("{number}\n -# unseeded").as_str());
        }
    };

    ctx.reply(response).await?;
    Ok(())
}
