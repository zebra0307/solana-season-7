//-------------------------------------------------------------------------------
///
/// TASK: Implement the remove reaction functionality for the Twitter program
/// 
/// Requirements:
/// - Verify that the tweet reaction exists and belongs to the reaction author
/// - Decrement the appropriate counter (likes or dislikes) on the tweet
/// - Close the tweet reaction account and return rent to reaction author
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn remove_reaction(ctx: Context<RemoveReactionContext>) -> Result<()> {
    // TODO: Implement remove reaction functionality
    //todo!()
    let tweet = &mut ctx.accounts.tweet;
    let tweet_reaction = &ctx.accounts.tweet_reaction;

    // 1. Decrement the appropriate counter on the tweet
    match tweet_reaction.reaction {
        ReactionType::Like => {
            if tweet.likes == 0 {
                return err!(TwitterError::MinLikesReached);
            }
            tweet.likes -= 1;
        }
        ReactionType::Dislike => {
            if tweet.dislikes == 0 {
                return err!(TwitterError::MinDislikesReached);
            }
            tweet.dislikes -= 1;
        }
    }

    // 2. The tweet reaction account is automatically closed and rent returned
    // to the reaction_author due to the `close` constraint.
    Ok(())

}

#[derive(Accounts)]
pub struct RemoveReactionContext<'info> {
    // TODO: Add required account constraints
    #[account(mut)]
    pub reaction_author: Signer<'info>,

    #[account(mut)]
    pub tweet: Account<'info, Tweet>,
    
     #[account(
        mut,
        close = reaction_author,
        has_one = reaction_author,
        seeds = [
            TWEET_REACTION_SEED.as_bytes(),
            reaction_author.key().as_ref(),
            tweet.key().as_ref()
        ],
        bump = tweet_reaction.bump
    )]
    pub tweet_reaction: Account<'info, Reaction>,
}
