use anchor_lang::prelude::*;

declare_id!("YourProgramIDWillGoHere11111111111111111111");

#[program]
pub mod note_app {
    use super::*;

    /// Creates a new user profile
    pub fn create_user(ctx: Context<CreateUser>, username: String) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        
        user_profile.authority = ctx.accounts.signer.key();
        user_profile.username = username;
        user_profile.note_count = 0;
        
        msg!("User profile created for: {}", user_profile.authority);
        Ok(())
    }

    /// Creates a new note for a user
    pub fn create_note(
        ctx: Context<CreateNote>,
        title: String,
        content: String,
    ) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        let note = &mut ctx.accounts.note;
        
        // Increment note count first
        user_profile.note_count += 1;
        
        // Set note data
        note.authority = ctx.accounts.signer.key();
        note.id = user_profile.note_count;
        note.title = title;
        note.content = content;
        
        msg!("Note #{} created successfully", note.id);
        Ok(())
    }

    /// Updates an existing note's content
    pub fn update_note(
        ctx: Context<UpdateNote>,
        _note_id: u64,
        new_content: String,
    ) -> Result<()> {
        let note = &mut ctx.accounts.note;
        note.content = new_content;
        
        msg!("Note #{} updated successfully", note.id);
        Ok(())
    }

    /// Deletes a note by closing the account
    pub fn delete_note(ctx: Context<DeleteNote>, _note_id: u64) -> Result<()> {
        msg!("Note #{} deleted successfully", ctx.accounts.note.id);
        Ok(())
    }
}

// =======================================================
// Context Structs (Define which accounts each instruction needs)
// =======================================================

#[derive(Accounts)]
#[instruction(username: String)]
pub struct CreateUser<'info> {
    #[account(
        init,
        payer = signer,
        space = 8 + UserProfile::INIT_SPACE,
        seeds = [b"user_profile", signer.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String, content: String)]
pub struct CreateNote<'info> {
    #[account(
        mut,
        seeds = [b"user_profile", signer.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub user_profile: Account<'info, UserProfile>,
    
    #[account(
        init,
        payer = signer,
        space = 8 + Note::INIT_SPACE,
        seeds = [
            b"note",
            signer.key().as_ref(),
            &(user_profile.note_count + 1).to_le_bytes()
        ],
        bump
    )]
    pub note: Account<'info, Note>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
    
    /// CHECK: This is safe because we verify it matches user_profile.authority
    pub authority: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(note_id: u64)]
pub struct UpdateNote<'info> {
    #[account(
        mut,
        seeds = [
            b"note",
            signer.key().as_ref(),
            &note_id.to_le_bytes()
        ],
        bump,
        has_one = authority,
        constraint = authority.key() == signer.key() @ ErrorCode::UnauthorizedAccess
    )]
    pub note: Account<'info, Note>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
    
    /// CHECK: This is verified through has_one constraint
    pub authority: UncheckedAccount<'info>,
}

#[derive(Accounts)]
#[instruction(note_id: u64)]
pub struct DeleteNote<'info> {
    #[account(
        mut,
        seeds = [
            b"note",
            signer.key().as_ref(),
            &note_id.to_le_bytes()
        ],
        bump,
        has_one = authority,
        constraint = authority.key() == signer.key() @ ErrorCode::UnauthorizedAccess,
        close = signer
    )]
    pub note: Account<'info, Note>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
    
    /// CHECK: This is verified through has_one constraint
    pub authority: UncheckedAccount<'info>,
}

// =======================================================
// Account Structs (Data stored on-chain)
// =======================================================

#[account]
#[derive(InitSpace)]
pub struct UserProfile {
    pub authority: Pubkey,      // 32 bytes
    #[max_len(50)]
    pub username: String,       // 4 + 50 bytes
    pub note_count: u64,        // 8 bytes
}

#[account]
#[derive(InitSpace)]
pub struct Note {
    pub authority: Pubkey,      // 32 bytes
    pub id: u64,                // 8 bytes
    #[max_len(100)]
    pub title: String,          // 4 + 100 bytes
    #[max_len(500)]
    pub content: String,        // 4 + 500 bytes
}

// =======================================================
// Error Codes
// =======================================================

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action")]
    UnauthorizedAccess,
}