use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, Mentionable, futures::StreamExt};

use crate::RTOC_GUILD;

const CORE_TEAM_ROLE: u64 = 1530912135037587609;
/// Timeout all members of a role, core team only.
#[poise::command(slash_command, prefix_command)]
pub async fn timeout_role(
    ctx: Context<'_>,
    #[description = "The role to timeout"] role: serenity::Role,
    #[description = "The duration to timeout, in seconds. Can't be longer than 14 days I thimk."]
    duration: i64,
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
    }

    ctx.defer().await?;

    let ctx_http = ctx.http();
    let mut members = serenity::GuildId::from(RTOC_GUILD)
        .members_iter(ctx_http)
        .boxed();

    let timestamp = serenity::Timestamp::from_unix_timestamp(
        serenity::Timestamp::now().unix_timestamp() + duration,
    );

    while let Some(member) = members.next().await {
        match member {
            Ok(mut member) => {
                if !member
                    .user
                    .has_role(&ctx, RTOC_GUILD, &role)
                    .await
                    .expect("error checking roles")
                {
                    continue;
                }

                match &timestamp {
                    Err(error) => eprintln!("error occurred calculating timestamp: {}", error),
                    Ok(timestamp) => {
                        member
                            .disable_communication_until_datetime(&ctx, *timestamp)
                            .await
                            .expect("error occurred timing out user");
                    }
                }
            }
            Err(error) => eprintln!("error occurred reading members: {error}"),
        }
    }
    ctx.send(
        poise::CreateReply::new()
            .content(format!(
                "timed out {} for {} seconds",
                role.mention(),
                duration
            ))
            .allowed_mentions(serenity::CreateAllowedMentions::new()),
    )
    .await?;
    Ok(())
}
