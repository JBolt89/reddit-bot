use rand::{RngExt, SeedableRng};

use crate::{Context, Error};

/// gambling :durr: (money resets when the bot restarts)
#[poise::command(slash_command, prefix_command)]
pub async fn gamble(ctx: Context<'_>) -> Result<(), Error> {
    let mut successes = -1;
    loop {
        let rng = rand::random_bool(0.5);
        if rng {
            successes += 1;
        } else {
            break;
        }
    }

    let money = {
        let mut hash_map = match ctx.data().money.lock() {
            Ok(h) => h,
            Err(e) => e.into_inner(),
        };

        let money = hash_map.entry(ctx.author().id.get()).or_insert(0);
        *money += successes;
        *money
    };

    ctx.reply(format!("gain/loss: ${successes}. you now has ${money}"))
        .await?;
    Ok(())
}

/// Like /gamble but you can give a seed. You don't earn anything from this.
#[poise::command(slash_command, prefix_command)]
pub async fn gamble_seeded(
    ctx: Context<'_>,
    #[description = "The seed to give the RNG"] seed: u64,
) -> Result<(), Error> {
    let mut successes = 0;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    loop {
        let result = rng.random_bool(0.5);

        if result {
            successes += 1;
        } else {
            break;
        }
    }

    ctx.reply(format!(
        "you would have gained/lost ${successes}, but this is seeded RNG"
    ))
    .await?;

    Ok(())
}
