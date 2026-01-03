use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod helpers;
pub mod instructions;
pub mod state;

use errors::HorseGameError;
use instructions::*;
use std::str::FromStr;

#[cfg(feature = "devnet")]
const ADMIN: &str = "6wJ1oVXBxGEp69i3Ay7Q4k3MxHqtdtrohRW1C4ucyV6S";
#[cfg(not(feature = "devnet"))]
const ADMIN: &str = "GqafiPLv32Tpeo6PVeVhq3Fk2pxL3QyyWLmabMCrn6j";

#[cfg(feature = "devnet")]
declare_id!("2S398gEigP71GhLjryxZwL4xTJ5iEE8QH3q5Vmo4r6v8");
#[cfg(not(feature = "devnet"))]
declare_id!("2S398gEigP71GhLjryxZwL4xTJ5iEE8QH3q5Vmo4r6v8");

#[program]
pub mod horsegame {
    use super::*;

    #[access_control(enforce_admin(ctx.accounts.authority.key))]
    pub fn initialize_program(
        ctx: Context<InitializeProgram>,
        start_slot: u64,
        total_supply: u64,
        initial_stable_purchase_fee_lamports: Option<u64>,
        horse_pack_cost_microtokens: Option<u64>,
        gamble_fee_lamports: Option<u64>,
        staking_lockup_slots: u64,
        token_reward_rate: u64,
    ) -> Result<()> {
        instructions::initialize_program(
            ctx,
            start_slot,
            total_supply,
            initial_stable_purchase_fee_lamports,
            horse_pack_cost_microtokens,
            gamble_fee_lamports,
            staking_lockup_slots,
            token_reward_rate,
        )
    }

    /// Set or update the token mint (admin only)
    /// Call this after initialize_program to configure the token
    /// Returns the vault address where you need to transfer tokens
    #[access_control(enforce_admin(ctx.accounts.authority.key))]
    pub fn set_token_mint(ctx: Context<SetTokenMint>) -> Result<()> {
        instructions::set_token_mint(ctx)
    }

    /// ────────────────────────────────────────────────────────────────────────────
    ///  ALL ADMIN FUNCTIONS ENFORCED BY AUTHORITY SIGNING IXS
    /// ────────────────────────────────────────────────────────────────────────────
    pub fn reset_player(ctx: Context<ResetPlayer>) -> Result<()> {
        instructions::reset_player(ctx)
    }

    pub fn toggle_production(ctx: Context<ToggleProduction>, enable: bool) -> Result<()> {
        instructions::toggle_production(ctx, enable)
    }

    pub fn update_pool_manual(ctx: Context<UpdatePool>) -> Result<()> {
        instructions::update_pool_manual(ctx)
    }

    pub fn update_parameter(
        ctx: Context<UpdateParameters>,
        parameter_index: u8,
        parameter_value: u64,
    ) -> Result<()> {
        instructions::update_parameter(ctx, parameter_index, parameter_value)
    }

    // ────────────────────────────────────────────────────────────────────────────
    ///  NON ADMIN FUNCTIONS
    // ────────────────────────────────────────────────────────────────────────────
    
    /// Purchase initial stable to start playing
    pub fn purchase_initial_stable(ctx: Context<PurchaseInitialStable>) -> Result<()> {
        instructions::purchase_initial_stable(ctx)
    }

    /// Enter a horse into racing (staking)
    pub fn enter_race(ctx: Context<EnterRace>, horse_index: u8) -> Result<()> {
        instructions::enter_race(ctx, horse_index)
    }

    /// Withdraw horse from racing
    pub fn withdraw_from_race(ctx: Context<WithdrawFromRace>, horse_index: u8) -> Result<()> {
        instructions::withdraw_from_race(ctx, horse_index)
    }

    /// Release a horse (remove from stable)
    pub fn release_horse(ctx: Context<ReleaseHorse>, horse_index: u8) -> Result<()> {
        instructions::release_horse(ctx, horse_index)
    }

    /// Commit to opening a horse pack
    pub fn open_horse_pack_commit(ctx: Context<OpenHorsePackCommit>) -> Result<()> {
        instructions::open_horse_pack_commit(ctx)
    }

    /// Settle horse pack opening (reveal horses)
    pub fn settle_open_horse_pack(ctx: Context<SettleOpenHorsePack>) -> Result<()> {
        instructions::settle_open_horse_pack(ctx)
    }

    /// Upgrade stable capacity
    pub fn upgrade_stable(ctx: Context<UpgradeStable>, stable_type: u8) -> Result<()> {
        instructions::upgrade_stable(ctx, stable_type)
    }

    /// Claim racing rewards
    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        instructions::claim_rewards(ctx)
    }

    /// Commit to breeding horses
    pub fn breed_horses_commit(
        ctx: Context<BreedHorsesCommit>,
        horse_indices: Vec<u8>,
    ) -> Result<()> {
        instructions::breed_horses_commit(ctx, horse_indices)
    }

    /// Settle breeding (reveal offspring)
    pub fn breed_horses_settle(ctx: Context<BreedHorsesSettle>) -> Result<()> {
        instructions::breed_horses_settle(ctx)
    }

    /// Cancel a pending action
    pub fn cancel_pending_action(ctx: Context<CancelPendingAction>) -> Result<()> {
        instructions::cancel_pending_action(ctx)
    }
}

fn enforce_admin(key: &Pubkey) -> Result<()> {
    #[cfg(not(feature = "test"))]
    require!(
        *key == Pubkey::from_str(ADMIN).unwrap(),
        HorseGameError::Unauthorized
    );
    Ok(())
}

