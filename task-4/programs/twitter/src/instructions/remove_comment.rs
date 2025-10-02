//-------------------------------------------------------------------------------
///
/// TASK: Implement the remove comment functionality for the Twitter program
/// 
/// Requirements:
/// - Close the comment account and return rent to comment author
/// 
/// NOTE: No implementation logic is needed in the function body - this 
/// functionality is achieved entirely through account constraints!
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

use crate::states::*;

pub fn remove_comment(_ctx: Context<RemoveCommentContext>, _comment_content: String) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
#[instruction(comment_content: String)]
pub struct RemoveCommentContext<'info> {
    // TODO: Add required account constraints
     #[account(mut)]
    pub comment_author: Signer<'info>,

     #[account(
        mut,
        has_one = comment_author,
        close = comment_author,
        seeds = [
            COMMENT_SEED.as_bytes(),
            comment_author.key().as_ref(),
            hash(comment_content.as_bytes()).as_ref(),
            tweet.key().as_ref()
        ],
        bump = comment.bump
    )]
    pub comment: Account<'info, Comment>,

    pub tweet: Account<'info, Tweet>,
}
