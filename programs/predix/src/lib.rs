use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;
use borsh::{BorshDeserialize, BorshSerialize};

declare_id!("FWp2NxpzgK235TFK1HE96rbVNtXZ8U2oxwXMn3ptwu1A");

#[program]
pub mod rehelloanchor {
    use super::*;

    pub fn create_post(
        ctx: Context<CreatePost>,
        topic: String,
        content: String,
        post_end_date: i64,
    ) -> Result<()> {
        let post: &mut Account<Post> = &mut ctx.accounts.post;
        let author: &Signer = &ctx.accounts.author;
        let clock = Clock::get().unwrap();

        // Char limit for strings
        if topic.chars().count() > 50 {
            return Err(ErrorCode::TopicTooLong.into());
        }
        if content.chars().count() > 280 {
            return Err(ErrorCode::ContentTooLong.into());
        }

        post.author = author.key();
        post.post_date = clock.unix_timestamp;
        post.post_end_date = post_end_date;
        post.topic = topic;
        post.content = content;
        post.post_init_bet = 0;
        post.post_bet_pool = 0;
        post.post_finished = false;

        post.post_participants.push(author.key());

        Ok(())
    }


    // New function to allow participation in a post
    pub fn participate_to_post(
        ctx: Context<ParticipateToPost>,
    ) -> Result<()> {
        let post: &mut Account<Post> = &mut ctx.accounts.post;
        let participant: &Signer = &ctx.accounts.participant;

        /* // Check if the post is already finished
        if post.post_finished {
            return Err(ErrorCode::PostAlreadyFinished.into());
        }

        // Check if the participant is already in the post_participants list
        if post.post_participants.contains(&participant.key()) {
            return Err(ErrorCode::AlreadyParticipated.into());
        }
 */
        // Add participant to the post
        post.post_participants.push(participant.key());

        Ok(())
    }

}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init, payer = author, space = Post::LEN)]
    pub post: Account<'info, Post>,
    #[account(mut)]
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub system_program: AccountInfo<'info>,
    //ppub system_program: Program<'info, System>,
}

#[account]
pub struct Post {
    pub author: Pubkey,                 //Post author
    pub post_date: i64,                 //Post publish date
    pub post_end_date: i64,             //Post end date
    pub topic: String,                  //Post topic
    pub content: String,                //Post content
    pub post_init_bet: u64,             //Post init bet
    pub post_bet_pool: u64,             //Post bet pool
    pub post_finished: bool,            //Post finished?
    pub post_participants: Vec<Pubkey>, //Post participants
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TOPIC_LENGTH: usize = 50 * 4;
const MAX_CONTENT_LENGTH: usize = 280 * 4;
const POST_BET_LENGTH: usize = 8;
const POST_FINISHED_LENGTH: usize = 1;
const POST_PARTICIPANTS_LENGTH: usize = 32 * 10;

impl Post {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH // Content.
        + POST_BET_LENGTH // Post date.
        + POST_BET_LENGTH // Post bet pool.
        + POST_FINISHED_LENGTH // Post status.
        + POST_PARTICIPANTS_LENGTH;
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided topic should be 50 characters long maximum.")]
    TopicTooLong,
    #[msg("The provided content should be 280 characters long maximum.")]
    ContentTooLong,
}

// --------------------------------------------------------CHATGPT-------------------------------------------------------------------- //
// Context struct for participate_to_post
#[derive(Accounts)]
pub struct ParticipateToPost<'info> {
    #[account(mut)]
    pub post: Account<'info, Post>, // Post to participate in
    #[account(mut)]
    pub participant: Signer<'info>, // Participant
    // System program declaration remains unchanged.
}
