use anchor_lang::prelude::*;

// declare_id 宏：定义程序 ID，是 /target/deploy/my_project-keypair.json 中生成的密钥对的公钥
declare_id!("FgPRhRgodU33QJRHoNmnBj1dhMNdYcoGEWaEVhCrzEBb");

// program 宏：定义一个 Anchor 程序模块
#[program]
pub mod stablecoin_contracts_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from==========================: {:?}", ctx.accounts);
        msg!("Greetings from==========================: {:?}", ctx.program_id);
        msg!("Greetings from==========================: {:?}", ctx.bumps);
        msg!("Greetings from==========================: {:?}", ctx.remaining_accounts);
        Ok(())
    }
}

// Accounts 宏：定义一个账户上下文结构体
#[derive(Accounts, Debug)]
pub struct Initialize {}
