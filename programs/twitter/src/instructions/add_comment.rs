use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_comment(
    ctx: Context<AddCommentContext>,
    comment_content: String,
) -> Result<()> {
    if comment_content.len() > CONTENT_LENGTH {
        return err!(TwitterError::ContentTooLong);
    }

    let comment = &mut ctx.accounts.comment;

    comment.comment_author = *ctx.accounts.comment_author.key;
    comment.parent_tweet = *ctx.accounts.parent_tweet.to_account_info().key;
    comment.content = comment_content;
    comment.bump = ctx.bumps.comment;

    Ok(())
}

#[derive(Accounts)]
#[instruction(comment_content: String)]
pub struct AddCommentContext<'info> {
    #[account(mut)]
    pub comment_author: Signer<'info>,

    #[account(
        init,
        payer = comment_author,
        space = Comment::INIT_SPACE,
        seeds = [
            COMMENT_SEED.as_bytes(),
            comment_author.key().as_ref(),
            hash(comment_content.as_bytes()).as_ref(),
            parent_tweet.key().as_ref()
        ],
        bump
    )]
    pub comment: Account<'info, Comment>,
    
    #[account(mut)]
    pub parent_tweet: Account<'info, Tweet>,
    
    pub system_program: Program<'info, System>,
}