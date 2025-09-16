use anchor_lang::prelude::*;

// 程序 ID，是 /target/deploy/my_project-keypair.json 中生成的密钥对的公钥
declare_id!("FgPRhRgodU33QJRHoNmnBj1dhMNdYcoGEWaEVhCrzEBb");

#[program]
pub mod stablecoin_contracts_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from-----------------------------------------------: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
