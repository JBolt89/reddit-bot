use poise::serenity_prelude as serenity;

use crate::ns_references::NULLSCAPE_REFERENCES;

pub async fn nullscape_check(
    ctx: serenity::prelude::Context,
    msg: poise::serenity_prelude::Message,
) {
    let refs = NULLSCAPE_REFERENCES
        .iter()
        .filter(|s| msg.content.to_lowercase().contains(*s))
        .count();

    if refs > 1 {
        msg.react(
            ctx,
            serenity::ReactionType::try_from("<:SubspaceTripmine:1560623715861926030>").unwrap(),
        )
        .await
        .expect("should have been able to react subspace tripmine");
    }
}
