use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_reaction(ctx: Context<AddReactionContext>, reaction: ReactionType) -> Result<()> {
    let tweet = &mut ctx.accounts.tweet;
    let tweet_reaction = &mut ctx.accounts.tweet_reaction;

    match reaction {
        ReactionType::Like => {
            if tweet.likes == u64::MAX {
                return err!(TwitterError::MaxLikesReached);
            }
            tweet.likes += 1;
        }
        ReactionType::Dislike => {
            if tweet.dislikes == u64::MAX {
                return err!(TwitterError::MaxDislikesReached);
            }
            tweet.dislikes += 1;
        }
    }

    tweet_reaction.reaction_author = *ctx.accounts.reaction_author.key;
    tweet_reaction.parent_tweet = *ctx.accounts.tweet.to_account_info().key;
    tweet_reaction.reaction = reaction;
    tweet_reaction.bump = ctx.bumps.tweet_reaction;

    Ok(())
}

#[derive(Accounts)]
pub struct AddReactionContext<'info> {
    #[account(mut)]
    pub reaction_author: Signer<'info>,

    #[account(mut)]
    pub tweet: Account<'info, Tweet>,

    #[account(
        init,
        payer = reaction_author,
        space = Reaction::INIT_SPACE + 8,
        seeds = [
            TWEET_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            tweet.key().as_ref()
        ],
        bump
    )]
    pub tweet_reaction: Account<'info, Reaction>,
    
    pub system_program: Program<'info, System>,
}