//-------------------------------------------------------------------------------
///
/// TASK: Implement the initialize tweet functionality for the Twitter program
/// 
/// Requirements:
/// - Validate that topic and content don't exceed maximum lengths
/// - Initialize a new tweet account with proper PDA seeds
/// - Set tweet fields: topic, content, author, likes, dislikes, and bump
/// - Initialize counters (likes and dislikes) to zero
/// - Use topic in PDA seeds for tweet identification
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn initialize_tweet(
    ctx: Context<InitializeTweet>,
    topic: String,
    content: String,
) -> Result<()> {
    // TODO: Implement initialize tweet functionality
    //todo!()
     // 1. Validate that topic and content don't exceed the maximum lengths.
    if topic.len() > TOPIC_LENGTH {
        return err!(TwitterError::TopicTooLong);
    }
    if content.len() > CONTENT_LENGTH {
        return err!(TwitterError::ContentTooLong);
    }

    // 2. Get the tweet account from the context.
    let tweet = &mut ctx.accounts.tweet;

    // 3. Set the tweet fields.
    tweet.tweet_author = *ctx.accounts.tweet_authority.key;
    tweet.topic = topic;
    tweet.content = content;
    tweet.likes = 0;
    tweet.dislikes = 0;
    tweet.bump = ctx.bumps.tweet;

    Ok(())
}

#[derive(Accounts)]
#[instruction(topic: String)]
pub struct InitializeTweet<'info> {
    // TODO: Add required account constraints
    #[account(mut)]
    pub tweet_authority: Signer<'info>,

    #[account(
        init,
        payer = tweet_authority,
        space = Tweet::INIT_SPACE + 8,
        seeds = [
            topic.as_bytes(),
            TWEET_SEED.as_bytes(),
            tweet_authority.key().as_ref()
        ],
        bump
    )]
    pub tweet: Account<'info, Tweet>,
    pub system_program: Program<'info, System>,
}
