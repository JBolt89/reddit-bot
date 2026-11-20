use crate::constants::*;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[derive(poise::ChoiceParameter)]
pub enum ActivityKind {
    #[name = "Playing"]
    Playing,
    #[name = "Watching"]
    Watching,
    #[name = "Listening"]
    Listening,
    #[name = "Competing"]
    Competing,
    #[name = "Streaming"]
    Streaming,
}

/// Sets the bot's activity. Doesn't persist through bot restarts.
#[poise::command(slash_command, prefix_command)]
pub async fn set_activity(
    ctx: Context<'_>,
    #[description = "The kind of activity."] kind: ActivityKind,
    #[description = "The text for the activity."] text: String,
) -> Result<(), Error> {
    if !ctx
        .author()
        .has_role(ctx, RTOC_GUILD, CORE_TEAM_ROLE)
        .await?
    {
        ctx.send(
            poise::CreateReply::new()
                .content("you need core team role you samsung fridge")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    match kind {
        ActivityKind::Playing => {
            ctx.framework()
                .serenity_context
                .set_activity(Some(serenity::ActivityData::playing(text)));
        }
        ActivityKind::Watching => {
            ctx.framework()
                .serenity_context
                .set_activity(Some(serenity::ActivityData::watching(text)));
        }
        ActivityKind::Listening => {
            ctx.framework()
                .serenity_context
                .set_activity(Some(serenity::ActivityData::listening(text)));
        }
        ActivityKind::Competing => {
            ctx.framework()
                .serenity_context
                .set_activity(Some(serenity::ActivityData::competing(text)));
        }
        ActivityKind::Streaming => {
            ctx.framework()
                .serenity_context
                .set_activity(Some(serenity::ActivityData::streaming(
                    text,
                    "https://discord.com/vanityurl/dotcom/steakpants/flour/flower/index11.html",
                )?));
        }
    }

    ctx.say("set the status of the bot").await?;
    Ok(())
}
