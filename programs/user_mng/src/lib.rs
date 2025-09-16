// ...existing code...
use anchor_lang::prelude::*;
declare_id!("ChUH8uuPprPLGAvQUzZMCX7u9CnxJ3VVQrcDbtVuSQdR");

#[program]
pub mod user_mng {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, owner: Pubkey) -> Result<()> {
        let user_management = &mut ctx.accounts.user_management;
        user_management.owner = owner;
        user_management.users = Vec::new();
        Ok(())
    }

    pub fn add_user(ctx: Context<ModifyUser>, user_address: Pubkey, name: String, description: String) -> Result<()> {
        let user_management = &mut ctx.accounts.user_management;
        // 如果已存在则更新，否则 push
        if let Some(idx) = user_management.users.iter().position(|e| e.key == user_address) {
            let entry = &mut user_management.users[idx];
            entry.user.name = name;
            entry.user.description = description;
            entry.user.is_whitelisted = true;
            entry.user.updated_at = Clock::get()?.unix_timestamp;
        } else {
            let now = Clock::get()?.unix_timestamp;
            user_management.users.push(UserEntry {
                key: user_address,
                user: User {
                    name,
                    description,
                    is_whitelisted: true,
                    fee_rate: 0,
                    created_at: now,
                    updated_at: now,
                },
            });
        }
        Ok(())
    }

    pub fn remove_user(ctx: Context<ModifyUser>, user_address: Pubkey) -> Result<()> {
        let user_management = &mut ctx.accounts.user_management;
        if let Some(idx) = user_management.users.iter().position(|e| e.key == user_address) {
            user_management.users.swap_remove(idx);
        }
        Ok(())
    }

    pub fn update_whitelist(ctx: Context<ModifyUser>, user_address: Pubkey, status: bool) -> Result<()> {
        let user_management = &mut ctx.accounts.user_management;
        if let Some(entry) = user_management.users.iter_mut().find(|e| e.key == user_address) {
            entry.user.is_whitelisted = status;
            entry.user.updated_at = Clock::get()?.unix_timestamp;
        }
        Ok(())
    }

    pub fn set_fee_rate(ctx: Context<ModifyUser>, user_address: Pubkey, fee_rate: u64) -> Result<()> {
        let user_management = &mut ctx.accounts.user_management;
        if let Some(entry) = user_management.users.iter_mut().find(|e| e.key == user_address) {
            entry.user.fee_rate = fee_rate;
            entry.user.updated_at = Clock::get()?.unix_timestamp;
        }
        Ok(())
    }
}

#[account]
pub struct UserManagement {
    pub owner: Pubkey,
    pub users: Vec<UserEntry>,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct User {
    pub name: String,
    pub description: String,
    pub is_whitelisted: bool,
    pub fee_rate: u64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct UserEntry {
    pub key: Pubkey,
    pub user: User,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32 + 10240)]
    pub user_management: Account<'info, UserManagement>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModifyUser<'info> {
    #[account(mut)]
    pub user_management: Account<'info, UserManagement>,
    pub user: Signer<'info>,
}
// ...existing code...