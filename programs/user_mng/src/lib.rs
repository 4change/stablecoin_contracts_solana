use anchor_lang::prelude::*;
declare_id!("ChUH8uuPprPLGAvQUzZMCX7u9CnxJ3VVQrcDbtVuSQdR");

#[program]
pub mod user_mng {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, owner: Pubkey) -> Result<()> {
        let cfg = &mut ctx.accounts.config;
        cfg.owner = owner;
        Ok(())
    }

    pub fn create_user(ctx: Context<CreateUser>, name: String, description: String) -> Result<()> {
        let user = &mut ctx.accounts.user_account;
        let now = Clock::get()?.unix_timestamp;
        user.owner = ctx.accounts.user_key.key();
        user.is_whitelisted = true;
        user.fee_rate_bps = 0;
        user.single_tx_limit = 0;
        user.annual_limit = 0;
        user.used_amount = 0;
        user.name = name;
        user.description = description;
        user.rcv_account = "".to_string();
        user.rcv_account_name = "".to_string();
        user.created_at = now;
        user.updated_at = now;
        Ok(())
    }

    pub fn update_user(
        ctx: Context<UpdateUser>,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<()> {
        let user = &mut ctx.accounts.user_account;
        if let Some(n) = name {
            user.name = n;
        }
        if let Some(d) = description {
            user.description = d;
        }
        user.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn set_fee_rate(ctx: Context<AdminOp>, fee_rate_bps: u64) -> Result<()> {
        let cfg = &ctx.accounts.config;
        require!(
            cfg.owner == ctx.accounts.authority.key(),
            ErrorCode::Unauthorized
        );
        let user = &mut ctx.accounts.user_account;
        user.fee_rate_bps = fee_rate_bps;
        user.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn update_whitelist(ctx: Context<AdminOp>, status: bool) -> Result<()> {
        let cfg = &ctx.accounts.config;
        require!(
            cfg.owner == ctx.accounts.authority.key(),
            ErrorCode::Unauthorized
        );
        let user = &mut ctx.accounts.user_account;
        user.is_whitelisted = status;
        user.updated_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn remove_user(ctx: Context<RemoveUser>) -> Result<()> {
        // closed to payer automatically by anchor via `close = payer`
        Ok(())
    }

    pub fn annual_limit_occupy(ctx: Context<TraderOp>, amount: u64) -> Result<()> {
        let cfg = &ctx.accounts.config;
        require!(
            cfg.owner == ctx.accounts.trader.key(),
            ErrorCode::Unauthorized
        );
        let user = &mut ctx.accounts.user_account;
        require!(user.is_whitelisted, ErrorCode::UserNotWhitelisted);
        require!(
            user.single_tx_limit >= amount,
            ErrorCode::SingleTxLimitExceeded
        );
        user.used_amount = user
            .used_amount
            .checked_add(amount)
            .ok_or(ErrorCode::Arithmetic)?;
        require!(
            user.used_amount <= user.annual_limit,
            ErrorCode::AnnualLimitExceeded
        );
        Ok(())
    }

    pub fn annual_limit_release(ctx: Context<TraderOp>, amount: u64) -> Result<()> {
        let cfg = &ctx.accounts.config;
        require!(
            cfg.owner == ctx.accounts.trader.key(),
            ErrorCode::Unauthorized
        );
        let user = &mut ctx.accounts.user_account;
        user.used_amount = user
            .used_amount
            .checked_sub(amount)
            .ok_or(ErrorCode::Arithmetic)?;
        Ok(())
    }
}

#[account]
pub struct Config {
    pub owner: Pubkey,
    // remove bump from stored state; Anchor will validate PDA via `bump` constraint
}

#[account]
pub struct UserAccount {
    pub owner: Pubkey,
    pub is_whitelisted: bool,
    pub fee_rate_bps: u64,
    pub single_tx_limit: u64,
    pub annual_limit: u64,
    pub used_amount: u64,
    pub name: String,
    pub description: String,
    pub rcv_account: String,
    pub rcv_account_name: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + 32, seeds = [b"config"], bump)]
    pub config: Account<'info, Config>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(init, payer = payer, space = 8 + 512, seeds = [b"user", user_key.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>,
    /// CHECK: only used as seed
    pub user_key: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateUser<'info> {
    #[account(mut, seeds = [b"user", user_key.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>,
    /// CHECK:
    pub user_key: UncheckedAccount<'info>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AdminOp<'info> {
    // validate PDA seeds using instruction bump (don't rely on stored bump)
    #[account(mut, seeds = [b"config"], bump)]
    pub config: Account<'info, Config>,
    pub authority: Signer<'info>,
    #[account(mut, seeds = [b"user", user_key.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>,
    /// CHECK:
    pub user_key: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct TraderOp<'info> {
    #[account(mut, seeds = [b"config"], bump)]
    pub config: Account<'info, Config>,
    pub trader: Signer<'info>,
    #[account(mut, seeds = [b"user", user_key.key().as_ref()], bump)]
    pub user_account: Account<'info, UserAccount>,
    /// CHECK:
    pub user_key: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct RemoveUser<'info> {
    #[account(mut, seeds = [b"user", user_key.key().as_ref()], bump, close = payer)]
    pub user_account: Account<'info, UserAccount>,
    /// CHECK:
    pub user_key: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("User not whitelisted")]
    UserNotWhitelisted,
    #[msg("Single tx limit exceeded")]
    SingleTxLimitExceeded,
    #[msg("Annual limit exceeded")]
    AnnualLimitExceeded,
    #[msg("Arithmetic error")]
    Arithmetic,
}
