use crate::{constants::*, errors::HorseGameError, helpers::*, state::*};
use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::slot_hashes;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Burn, Mint, TokenAccount, TokenInterface},
};

#[event]
pub struct StableUpgraded {
    pub player: Pubkey,
    pub new_stable_type: u8,
}

#[event]
pub struct HorseEnteredRace {
    pub player: Pubkey,
    pub horse_index: u8,
}

#[event]
pub struct HorseWithdrawnFromRace {
    pub player: Pubkey,
    pub horse_index: u8,
}

#[event]
pub struct HorseReleased {
    pub player: Pubkey,
    pub horse_index: u8,
}

#[event]
pub struct HorsePackOpened {
    pub player: Pubkey,
    pub horse_ids: [u8; 5],
}

#[event]
pub struct HorsesBred {
    pub player: Pubkey,
    pub successful_offspring: u8,
    pub total_bred: u8,
}

/// ────────────────────────────────────────────────────────────────────────────
/// INTERNAL: update the global accumulator
/// ────────────────────────────────────────────────────────────────────────────
fn update_pool(gs: &mut GlobalState, slot_now: u64) {
    if slot_now < gs.start_slot {
        gs.last_reward_slot = gs.start_slot;
        return;
    }

    if slot_now <= gs.last_reward_slot || gs.total_speed == 0 {
        gs.last_reward_slot = slot_now;
        return;
    }

    let rate_now = calculate_current_reward_rate(slot_now, gs.start_slot);

    let minted_minus_burn = gs.cumulative_rewards.saturating_sub(gs.burned_tokens);
    let remaining_supply = gs.total_supply.saturating_sub(minted_minus_burn);

    let dust_threshold = if gs.dust_threshold_divisor > 0 {
        gs.total_supply / gs.dust_threshold_divisor
    } else {
        0
    };

    if remaining_supply <= dust_threshold || rate_now == 0 {
        gs.last_reward_slot = slot_now;
        return;
    }

    let slots_elapsed = (slot_now - gs.last_reward_slot) as u128;
    let mut reward = slots_elapsed
        .checked_mul(rate_now as u128)
        .unwrap_or(u128::MAX);
    reward = reward.min(remaining_supply as u128);

    gs.acc_tokens_per_speed += reward * ACC_SCALE / gs.total_speed as u128;
    gs.cumulative_rewards = gs.cumulative_rewards.saturating_add(reward as u64);
    gs.reward_rate = rate_now;

    gs.last_reward_slot = slot_now;
}

/// Helper to settle and transfer rewards for a player
/// Rewards are capped by actual vault balance to prevent failed transfers
fn settle_and_mint_rewards<'info>(
    player: &mut Box<Account<'info, Player>>,
    gs: &mut Account<'info, GlobalState>,
    now: u64,
    player_token_account: &AccountInfo<'info>,
    token_mint: &AccountInfo<'info>,
    rewards_vault: &InterfaceAccount<'info, TokenAccount>,
    token_program: &AccountInfo<'info>,
    global_state_bump: u8,
    decimals: u8,
) -> Result<u64> {
    update_pool(gs, now);

    if now <= gs.start_slot {
        player.last_claim_slot = now;
        return Ok(0);
    }

    require!(
        now > player.last_claim_slot,
        HorseGameError::CooldownNotExpired
    );

    let pending_u128 = (player.total_speed as u128)
        .checked_mul(
            gs.acc_tokens_per_speed
                .saturating_sub(player.last_acc_tokens_per_speed),
        )
        .unwrap_or(u128::MAX)
        / ACC_SCALE;
    let mut pending = pending_u128 as u64;

    // Cap by theoretical remaining supply
    let minted_minus_burn = gs.cumulative_rewards.saturating_sub(gs.burned_tokens);
    let remaining_supply = gs.total_supply.saturating_sub(minted_minus_burn);
    if pending > remaining_supply {
        pending = remaining_supply;
    }

    // ⚠️ IMPORTANT: Cap by actual vault balance to prevent failed transfers
    let vault_balance = rewards_vault.amount;
    if pending > vault_balance {
        pending = vault_balance;
        msg!("⚠️ Rewards capped to vault balance: {}", vault_balance);
    }

    player.last_claim_slot = now;
    player.last_acc_tokens_per_speed = gs.acc_tokens_per_speed;

    if pending == 0 {
        return Ok(0);
    }

    let player_amount = pending;
    player.total_rewards = player.total_rewards.saturating_add(player_amount);

    // GlobalState PDA is derived with just [GLOBAL_STATE_SEED], so signing must match
    let seeds = &[
        GLOBAL_STATE_SEED,
        &[global_state_bump],
    ];
    let signer = &[&seeds[..]];

    token_interface::transfer_checked(
        CpiContext::new_with_signer(
            token_program.clone(),
            token_interface::TransferChecked {
                from: rewards_vault.to_account_info(),
                mint: token_mint.clone(),
                to: player_token_account.clone(),
                authority: gs.to_account_info(),
            },
            signer,
        ),
        player_amount,
        decimals,
    )?;

    Ok(pending)
}

/// ────────────────────────────────────────────────────────────────────────────
/// INITIALIZE PROGRAM (No token required - set later with set_token_mint)
/// ────────────────────────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct InitializeProgram<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        // Space: 8 (disc) + 32*4 (pubkeys) + 1 (bool) + 8*5 (u64) + 16 (u128) + 8 (u64) + 1*3 (u8s) + 8*13 (u64) + 16*2 (u128) + 8*3 (u64) + 64 (padding) = 436
        space = 8 + 32 + 32 + 32 + 32 + 1 + 8 + 8 + 8 + 8 + 8 + 16 + 8 + 1 + 1 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 16 + 16 + 8 + 8 + 8 + 64,
        seeds=[GLOBAL_STATE_SEED],  // Fixed seed - no mint dependency!
        bump
    )]
    pub global_state: Box<Account<'info, GlobalState>>,
    /// CHECK: This is the fees recipient wallet
    pub fees_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

/// Event emitted when program is initialized
#[event]
pub struct ProgramInitialized {
    pub authority: Pubkey,
    pub fees_wallet: Pubkey,
    pub start_slot: u64,
}

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
    let gs = &mut ctx.accounts.global_state;

    gs.authority = ctx.accounts.authority.key();
    gs.fees_wallet = ctx.accounts.fees_wallet.key();
    
    // Token not set yet - will be set via set_token_mint
    gs.token_mint = Pubkey::default();
    gs.rewards_vault = Pubkey::default();
    gs.token_initialized = false;

    gs.total_supply = total_supply;
    gs.burned_tokens = 0;
    gs.cumulative_rewards = 0;
    gs.start_slot = start_slot;
    gs.acc_tokens_per_speed = 0;
    gs.last_reward_slot = start_slot;

    gs.burn_rate = 80;
    gs.referral_fee = 100;
    gs.production_enabled = true;
    gs.dust_threshold_divisor = 1000;

    gs.initial_stable_purchase_fee_lamports =
        initial_stable_purchase_fee_lamports.unwrap_or(300_000_000); // 0.3 SOL
    gs.horse_pack_cost_microtokens = horse_pack_cost_microtokens.unwrap_or(200_000_000); // 200 tokens
    gs.gamble_fee_lamports = gamble_fee_lamports.unwrap_or(100_000_000); // 0.1 SOL

    gs.total_feed_consumption = 0;
    gs.total_speed = 0;

    gs.total_horse_packs_opened = 0;
    gs.total_breeding_attempts = 0;
    gs.total_successful_breeding = 0;

    gs.total_staked_tokens = 0;
    gs.staking_lockup_slots = staking_lockup_slots;
    gs.acc_sol_rewards_per_token = 0;
    gs.acc_token_rewards_per_token = 0;
    gs.last_staking_reward_slot = start_slot;
    gs.token_reward_rate = token_reward_rate;
    gs.total_sol_deposited = 0;

    emit!(ProgramInitialized {
        authority: gs.authority,
        fees_wallet: gs.fees_wallet,
        start_slot,
    });

    msg!("============================================");
    msg!("HORSEGAME INITIALIZED!");
    msg!("============================================");
    msg!("Authority: {}", gs.authority);
    msg!("Fees Wallet: {}", gs.fees_wallet);
    msg!("Total Supply: {}", total_supply);
    msg!("============================================");
    msg!(">>> Next step: Call set_token_mint() to configure the token and get the vault address");
    msg!("============================================");

    Ok(())
}

/// ────────────────────────────────────────────────────────────────────────────
/// SET TOKEN MINT (Admin only - can be called to set/change the token)
/// ────────────────────────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct SetTokenMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// The new token mint (Token-2022 from pump.fun)
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        has_one = authority @ HorseGameError::Unauthorized
    )]
    pub global_state: Account<'info, GlobalState>,
    /// CHECK: This is the fees recipient wallet from global_state
    #[account(
        constraint = fees_wallet.key() == global_state.fees_wallet @ HorseGameError::Unauthorized
    )]
    pub fees_wallet: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = token_mint,
        associated_token::authority = fees_wallet,
        associated_token::token_program = token_program,
    )]
    pub fees_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = global_state,
        token::token_program = token_program,
        seeds = [REWARDS_VAULT_SEED, token_mint.key().as_ref()],
        bump
    )]
    pub rewards_vault: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

/// Event emitted when token mint is set/updated
#[event]
pub struct TokenMintSet {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub rewards_vault: Pubkey,  // <-- THIS IS WHERE YOU TRANSFER TOKENS
    pub fees_wallet: Pubkey,
    pub is_update: bool,  // true if this is changing an existing mint
}

pub fn set_token_mint(ctx: Context<SetTokenMint>) -> Result<()> {
    let gs = &mut ctx.accounts.global_state;
    let was_initialized = gs.token_initialized;
    
    // Update the token mint and vault
    gs.token_mint = ctx.accounts.token_mint.key();
    gs.rewards_vault = ctx.accounts.rewards_vault.key();
    gs.token_initialized = true;

    emit!(TokenMintSet {
        authority: gs.authority,
        token_mint: gs.token_mint,
        rewards_vault: ctx.accounts.rewards_vault.key(),
        fees_wallet: gs.fees_wallet,
        is_update: was_initialized,
    });

    msg!("============================================");
    if was_initialized {
        msg!("TOKEN MINT UPDATED!");
    } else {
        msg!("TOKEN MINT SET!");
    }
    msg!("============================================");
    msg!("Token Mint: {}", ctx.accounts.token_mint.key());
    msg!("============================================");
    msg!(">>> REWARDS VAULT ADDRESS (send tokens here):");
    msg!(">>> {}", ctx.accounts.rewards_vault.key());
    msg!("============================================");
    msg!("Transfer your pump.fun tokens to this vault address!");
    msg!("============================================");

    Ok(())
}

/// ────────────────────────────────────────────────────────────────────────────
/// PURCHASE INITIAL STABLE
/// ────────────────────────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct PurchaseInitialStable<'info> {
    #[account(mut)]
    pub player_wallet: Signer<'info>,
    /// Token mint (validated to match global_state.token_mint)
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        constraint = global_state.token_initialized @ HorseGameError::TokenNotInitialized,
        constraint = global_state.token_mint == token_mint.key() @ HorseGameError::InvalidTokenMint,
    )]
    pub global_state: Box<Account<'info, GlobalState>>,
    #[account(
        init,
        payer = player_wallet,
        space = 8 + 32 + 10 + (MAX_HORSES_PER_PLAYER as usize * 6) + 1 + 16 + 8 + 8 + 33 + 16 + 8 + 8 + 8 + 8 + 8 + 130 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 16 + 16 + 8 + 8 + 8 + 64,
        seeds = [PLAYER_SEED, player_wallet.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub player: Box<Account<'info, Player>>,
    /// CHECK: This is the fees recipient wallet from global_state
    #[account(
        mut,
        constraint = fees_wallet.key() == global_state.fees_wallet @ HorseGameError::Unauthorized
    )]
    pub fees_wallet: AccountInfo<'info>,
    /// CHECK: This is the referrer's wallet
    #[account(mut)]
    pub referrer_wallet: Option<AccountInfo<'info>>,
    #[account(
        init_if_needed,
        payer = player_wallet,
        associated_token::mint = token_mint,
        associated_token::authority = player_wallet,
    )]
    pub player_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct InitialStablePurchased {
    pub player_wallet: Pubkey,
    pub player_account: Pubkey,
    pub referrer: Option<Pubkey>,
    pub stable_type: u8,
    pub initial_horses: u8,
    pub slot: u64,
}

pub fn purchase_initial_stable(ctx: Context<PurchaseInitialStable>) -> Result<()> {
    let slot = Clock::get()?.slot;
    let player = &mut ctx.accounts.player;
    let gs = &mut ctx.accounts.global_state;

    require!(gs.production_enabled, HorseGameError::ProductionDisabled);
    require!(
        player.horse_count == 0,
        HorseGameError::InitialStableAlreadyPurchased
    );

    let referrer: Option<Pubkey> = ctx.accounts.referrer_wallet.as_ref().map(|acc| acc.key());

    if let Some(ref r) = referrer {
        require!(
            *r != ctx.accounts.player_wallet.key(),
            HorseGameError::SelfReferralNotAllowed
        );
    }

    update_pool(gs, slot);

    // Transfer SOL fee
    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.player_wallet.to_account_info(),
                to: ctx.accounts.fees_wallet.to_account_info(),
            },
        ),
        gs.initial_stable_purchase_fee_lamports,
    )?;

    // Initialize player
    player.owner = ctx.accounts.player_wallet.key();
    let (racing_slots, feed_capacity, _) = STABLE_CONFIGS[1];
    player.stable = Stable {
        stable_type: 1,
        racing_slots,
        feed_capacity,
    };

    player.horses = [Horse::default(); MAX_HORSES_PER_PLAYER as usize];
    player.horse_count = 0;
    player.racing_horses_bitset = 0;

    // Give player 3 starter horses
    for &horse_id in STARTER_HORSE_IDS.iter() {
        if let Some((grade, speed, stamina_cost)) = get_horse_by_id(horse_id) {
            let horse = Horse {
                id: horse_id,
                grade,
                speed,
                stamina_cost,
            };
            player.add_horse(horse)?;
        }
    }

    player.feed_consumption = 0;
    player.total_speed = 0;
    player.referrer = referrer;
    player.last_claim_slot = slot;
    player.last_upgrade_slot = slot;
    player.total_rewards = 0;
    player.last_acc_tokens_per_speed = gs.acc_tokens_per_speed;

    player.total_gambles = 0;
    player.total_gamble_wins = 0;
    player.pending_action = PendingRandomAction::None;
    player.commit_slot = 0;

    player.total_earnings_for_referrer = 0;
    player.total_horse_packs_opened = 0;
    player.total_horses_bred = 0;
    player.successful_breeding = 0;
    player.total_sol_spent = gs.initial_stable_purchase_fee_lamports;
    player.total_tokens_spent = 0;

    player.staked_tokens = 0;
    player.last_stake_slot = 0;
    player.last_acc_sol_rewards_per_token = 0;
    player.last_acc_token_rewards_per_token = 0;
    player.claimed_token_rewards = 0;

    player.total_races_entered = 0;
    player.total_race_wins = 0;

    player.padding = [0u8; 64];

    emit!(InitialStablePurchased {
        player_wallet: ctx.accounts.player_wallet.key(),
        player_account: player.key(),
        referrer,
        stable_type: player.stable.stable_type,
        initial_horses: player.horse_count,
        slot,
    });

    Ok(())
}

/// ────────────────────────────────────────────────────────────────────────────
/// RELEASE HORSE
/// ────────────────────────────────────────────────────────────────────────────
#[derive(Accounts)]
#[instruction(horse_index: u8)]
pub struct ReleaseHorse<'info> {
    #[account(mut)]
    pub player_wallet: Signer<'info>,
    #[account(
        mut,
        constraint = player.owner == player_wallet.key() @ HorseGameError::Unauthorized,
        constraint = player.pending_action == PendingRandomAction::None @ HorseGameError::HorsePendingBreeding,
        seeds = [PLAYER_SEED, player_wallet.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub player: Box<Account<'info, Player>>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        constraint = global_state.token_initialized @ HorseGameError::TokenNotInitialized,
        constraint = global_state.token_mint == token_mint.key() @ HorseGameError::InvalidTokenMint,
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        seeds = [REWARDS_VAULT_SEED, token_mint.key().as_ref()],
        bump,
    )]
    pub rewards_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = player_token_account.mint == global_state.token_mint,
        constraint = player_token_account.owner == player_wallet.key() @ HorseGameError::InvalidTokenAccountOwner
    )]
    pub player_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = fees_token_account.mint == global_state.token_mint,
        constraint = fees_token_account.owner == global_state.fees_wallet @ HorseGameError::Unauthorized
    )]
    pub fees_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account()]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn release_horse(ctx: Context<ReleaseHorse>, horse_index: u8) -> Result<()> {
    let slot = Clock::get()?.slot;
    let player = &mut ctx.accounts.player;
    let gs = &mut ctx.accounts.global_state;

    require!(slot >= gs.start_slot, HorseGameError::ProductionDisabled);
    require!(gs.production_enabled, HorseGameError::ProductionDisabled);

    validate_horse_index(horse_index, player.horse_count as usize)?;

    require!(
        !player.is_horse_racing(horse_index),
        HorseGameError::HorseIsRacing
    );

    settle_and_mint_rewards(
        player,
        gs,
        slot,
        &ctx.accounts.player_token_account.to_account_info(),
        &ctx.accounts.token_mint.to_account_info(),
        &ctx.accounts.rewards_vault,
        &ctx.accounts.token_program.to_account_info(),
        ctx.bumps.global_state,
        ctx.accounts.token_mint.decimals,
    )?;

    player.batch_remove_horses(&[horse_index])?;

    emit!(HorseReleased {
        player: player.key(),
        horse_index,
    });

    Ok(())
}

/// ────────────────────────────────────────────────────────────────────────────
/// ENTER RACE (stake horse)
/// ────────────────────────────────────────────────────────────────────────────
#[derive(Accounts)]
#[instruction(horse_index: u8)]
pub struct EnterRace<'info> {
    #[account(mut)]
    pub player_wallet: Signer<'info>,
    #[account(
        mut,
        constraint = player.owner == player_wallet.key() @ HorseGameError::Unauthorized,
        constraint = player.pending_action == PendingRandomAction::None @ HorseGameError::HorsePendingBreeding,
        seeds = [PLAYER_SEED, player_wallet.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub player: Box<Account<'info, Player>>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        constraint = global_state.token_initialized @ HorseGameError::TokenNotInitialized,
        constraint = global_state.token_mint == token_mint.key() @ HorseGameError::InvalidTokenMint,
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        seeds = [REWARDS_VAULT_SEED, token_mint.key().as_ref()],
        bump,
    )]
    pub rewards_vault: InterfaceAccount<'info, TokenAccount>,
    #[account()]
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        constraint = player_token_account.mint == global_state.token_mint,
        constraint = player_token_account.owner == player_wallet.key() @ HorseGameError::InvalidTokenAccountOwner
    )]
    pub player_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn enter_race(ctx: Context<EnterRace>, horse_index: u8) -> Result<()> {
    let slot = Clock::get()?.slot;
    let player = &mut ctx.accounts.player;
    let gs = &mut ctx.accounts.global_state;

    settle_and_mint_rewards(
        player,
        gs,
        slot,
        &ctx.accounts.player_token_account.to_account_info(),
        &ctx.accounts.token_mint.to_account_info(),
        &ctx.accounts.rewards_vault,
        &ctx.accounts.token_program.to_account_info(),
        ctx.bumps.global_state,
        ctx.accounts.token_mint.decimals,
    )?;

    validate_horse_index(horse_index, player.horse_count as usize)?;

    require!(
        !player.is_horse_racing(horse_index),
        HorseGameError::HorseIsRacing
    );

    require!(
        player.count_racing_horses() < player.stable.racing_slots,
        HorseGameError::StableCapacityExceeded
    );

    let horse = &player.horses[horse_index as usize];
    let horse_stamina = horse.stamina_cost as u64;
    let horse_speed = horse.speed as u64;

    let new_player_feed = safe_add_feed(player.feed_consumption, horse_stamina)?;
    let new_total_feed = safe_add_feed(gs.total_feed_consumption, horse_stamina)?;
    let new_player_speed = safe_add_speed(player.total_speed, horse_speed)?;
    let new_total_speed = safe_add_speed(gs.total_speed, horse_speed)?;

    require!(
        new_player_feed <= player.stable.feed_capacity,
        HorseGameError::FeedCapacityExceeded
    );

    player.enter_horse_in_race(horse_index)?;
    player.feed_consumption = new_player_feed;
    player.total_speed = new_player_speed;
    player.total_races_entered = player.total_races_entered.saturating_add(1);
    gs.total_feed_consumption = new_total_feed;
    gs.total_speed = new_total_speed;

    emit!(HorseEnteredRace {
        player: player.key(),
        horse_index,
    });

    Ok(())
}

/// ────────────────────────────────────────────────────────────────────────────
/// WITHDRAW FROM RACE (unstake horse)
/// ────────────────────────────────────────────────────────────────────────────
#[derive(Accounts)]
#[instruction(horse_index: u8)]
pub struct WithdrawFromRace<'info> {
    #[account(mut)]
    pub player_wallet: Signer<'info>,
    #[account(
        mut,
        constraint = player.owner == player_wallet.key() @ HorseGameError::Unauthorized,
        constraint = player.pending_action == PendingRandomAction::None @ HorseGameError::HorsePendingBreeding,
        seeds = [PLAYER_SEED, player_wallet.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub player: Box<Account<'info, Player>>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        constraint = global_state.token_initialized @ HorseGameError::TokenNotInitialized,
        constraint = global_state.token_mint == token_mint.key() @ HorseGameError::InvalidTokenMint,
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        seeds = [REWARDS_VAULT_SEED, token_mint.key().as_ref()],
        bump,
    )]
    pub rewards_vault: InterfaceAccount<'info, TokenAccount>,
    #[account()]
    pub token_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        constraint = player_token_account.mint == global_state.token_mint,
        constraint = player_token_account.owner == player_wallet.key() @ HorseGameError::InvalidTokenAccountOwner
    )]
    pub player_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn withdraw_from_race(ctx: Context<WithdrawFromRace>, horse_index: u8) -> Result<()> {
    let slot = Clock::get()?.slot;
    let player = &mut ctx.accounts.player;
    let gs = &mut ctx.accounts.global_state;

    settle_and_mint_rewards(
        player,
        gs,
        slot,
        &ctx.accounts.player_token_account.to_account_info(),
        &ctx.accounts.token_mint.to_account_info(),
        &ctx.accounts.rewards_vault,
        &ctx.accounts.token_program.to_account_info(),
        ctx.bumps.global_state,
        ctx.accounts.token_mint.decimals,
    )?;

    validate_horse_index(horse_index, player.horse_count as usize)?;

    require!(
        player.is_horse_racing(horse_index),
        HorseGameError::HorseNotRacing
    );

    let horse = &player.horses[horse_index as usize];
    let horse_stamina = horse.stamina_cost as u64;
    let horse_speed = horse.speed as u64;

    let new_player_feed = safe_sub_feed(player.feed_consumption, horse_stamina)?;
    let new_total_feed = safe_sub_feed(gs.total_feed_consumption, horse_stamina)?;
    let new_player_speed = safe_sub_speed(player.total_speed, horse_speed)?;
    let new_total_speed = safe_sub_speed(gs.total_speed, horse_speed)?;

    player.withdraw_horse_from_race(horse_index)?;
    player.feed_consumption = new_player_feed;
    player.total_speed = new_player_speed;
    gs.total_feed_consumption = new_total_feed;
    gs.total_speed = new_total_speed;

    emit!(HorseWithdrawnFromRace {
        player: player.key(),
        horse_index,
    });

    Ok(())
}

/// ────────────────────────────────────────────────────────────────────────────
/// UPGRADE STABLE
/// ────────────────────────────────────────────────────────────────────────────
#[derive(Accounts)]
#[instruction(stable_type: u8)]
pub struct UpgradeStable<'info> {
    #[account(mut)]
    pub player_wallet: Signer<'info>,
    #[account(
        mut,
        constraint = player.owner == player_wallet.key() @ HorseGameError::Unauthorized,
        constraint = player.stable.stable_type + 1 == stable_type && (stable_type as usize) <= STABLE_CONFIGS.len() - 1 @ HorseGameError::InvalidStableType,
        seeds = [PLAYER_SEED, player_wallet.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub player: Box<Account<'info, Player>>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        constraint = global_state.token_initialized @ HorseGameError::TokenNotInitialized,
        constraint = global_state.token_mint == token_mint.key() @ HorseGameError::InvalidTokenMint,
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        seeds = [REWARDS_VAULT_SEED, token_mint.key().as_ref()],
        bump,
    )]
    pub rewards_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = player_token_account.mint == global_state.token_mint,
        constraint = player_token_account.owner == player_wallet.key() @ HorseGameError::InvalidTokenAccountOwner
    )]
    pub player_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = fees_token_account.mint == global_state.token_mint,
        constraint = fees_token_account.owner == global_state.fees_wallet @ HorseGameError::Unauthorized
    )]
    pub fees_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn upgrade_stable(ctx: Context<UpgradeStable>, stable_type: u8) -> Result<()> {
    let slot = Clock::get()?.slot;
    let player = &mut ctx.accounts.player;
    let gs = &mut ctx.accounts.global_state;

    require!(gs.production_enabled, HorseGameError::ProductionDisabled);

    settle_and_mint_rewards(
        player,
        gs,
        slot,
        &ctx.accounts.player_token_account.to_account_info(),
        &ctx.accounts.token_mint.to_account_info(),
        &ctx.accounts.rewards_vault,
        &ctx.accounts.token_program.to_account_info(),
        ctx.bumps.global_state,
        ctx.accounts.token_mint.decimals,
    )?;

    let (racing_slots, feed_capacity, cost) = STABLE_CONFIGS[stable_type as usize];

    require!(
        ctx.accounts.player_token_account.amount >= cost,
        HorseGameError::InsufficientTokens
    );

    player.stable.stable_type = stable_type;
    player.stable.racing_slots = racing_slots;
    player.stable.feed_capacity = feed_capacity;
    player.last_upgrade_slot = slot;
    player.total_tokens_spent = player.total_tokens_spent.saturating_add(cost);

    handle_fee_transfers(
        player,
        gs,
        cost,
        &ctx.accounts.player_token_account.to_account_info(),
        &ctx.accounts.fees_token_account.to_account_info(),
        None,
        &ctx.accounts.player_wallet.to_account_info(),
        &ctx.accounts.token_program.to_account_info(),
        &ctx.accounts.token_mint.to_account_info(),
        false,
        ctx.accounts.token_mint.decimals,
    )?;

    emit!(StableUpgraded {
        player: ctx.accounts.player_wallet.key(),
        new_stable_type: stable_type,
    });

    Ok(())
}

/// ────────────────────────────────────────────────────────────────────────────
/// CLAIM REWARDS
/// ────────────────────────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub player_wallet: Signer<'info>,
    #[account(
        mut,
        constraint = player.owner == player_wallet.key() @ HorseGameError::Unauthorized,
        seeds = [PLAYER_SEED, player_wallet.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub player: Box<Account<'info, Player>>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        constraint = global_state.token_initialized @ HorseGameError::TokenNotInitialized,
        constraint = global_state.token_mint == token_mint.key() @ HorseGameError::InvalidTokenMint,
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        seeds = [REWARDS_VAULT_SEED, token_mint.key().as_ref()],
        bump,
    )]
    pub rewards_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = player_token_account.owner == player_wallet.key(),
        constraint = player_token_account.mint == global_state.token_mint
    )]
    pub player_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account()]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
    let now = Clock::get()?.slot;

    settle_and_mint_rewards(
        &mut ctx.accounts.player,
        &mut ctx.accounts.global_state,
        now,
        &ctx.accounts.player_token_account.to_account_info(),
        &ctx.accounts.token_mint.to_account_info(),
        &ctx.accounts.rewards_vault,
        &ctx.accounts.token_program.to_account_info(),
        ctx.bumps.global_state,
        ctx.accounts.token_mint.decimals,
    )?;

    Ok(())
}

/// ────────────────────────────────────────────────────────────────────────────
/// OPEN HORSE PACK (commit phase)
/// ────────────────────────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct OpenHorsePackCommit<'info> {
    #[account(mut)]
    pub player_wallet: Signer<'info>,
    #[account(
        mut,
        constraint = player.owner == player_wallet.key() @ HorseGameError::Unauthorized,
        constraint = player.pending_action == PendingRandomAction::None @ HorseGameError::HorsePackAlreadyPending,
        seeds = [PLAYER_SEED, player_wallet.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub player: Box<Account<'info, Player>>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        constraint = global_state.token_initialized @ HorseGameError::TokenNotInitialized,
        constraint = global_state.token_mint == token_mint.key() @ HorseGameError::InvalidTokenMint,
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        seeds = [REWARDS_VAULT_SEED, token_mint.key().as_ref()],
        bump,
    )]
    pub rewards_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = player_token_account.mint == global_state.token_mint,
        constraint = player_token_account.owner == player_wallet.key() @ HorseGameError::InvalidTokenAccountOwner
    )]
    pub player_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = fees_token_account.mint == global_state.token_mint,
        constraint = fees_token_account.owner == global_state.fees_wallet @ HorseGameError::Unauthorized
    )]
    pub fees_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub referrer_token_account: Option<InterfaceAccount<'info, TokenAccount>>,
    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn open_horse_pack_commit(ctx: Context<OpenHorsePackCommit>) -> Result<()> {
    let slot = Clock::get()?.slot;
    let player = &mut ctx.accounts.player;
    let gs = &mut ctx.accounts.global_state;

    require!(gs.production_enabled, HorseGameError::ProductionDisabled);
    require!(
        (player.horse_count as usize) + 5 <= MAX_HORSES_PER_PLAYER as usize,
        HorseGameError::StableCapacityExceeded
    );

    settle_and_mint_rewards(
        player,
        gs,
        slot,
        &ctx.accounts.player_token_account.to_account_info(),
        &ctx.accounts.token_mint.to_account_info(),
        &ctx.accounts.rewards_vault,
        &ctx.accounts.token_program.to_account_info(),
        ctx.bumps.global_state,
        ctx.accounts.token_mint.decimals,
    )?;

    let pack_cost = gs.horse_pack_cost_microtokens;

    handle_fee_transfers(
        player,
        gs,
        pack_cost,
        &ctx.accounts.player_token_account.to_account_info(),
        &ctx.accounts.fees_token_account.to_account_info(),
        ctx.accounts.referrer_token_account.clone(),
        &ctx.accounts.player_wallet.to_account_info(),
        &ctx.accounts.token_program.to_account_info(),
        &ctx.accounts.token_mint.to_account_info(),
        true,
        ctx.accounts.token_mint.decimals,
    )?;

    player.pending_action = PendingRandomAction::HorsePack;
    player.commit_slot = slot;
    player.total_tokens_spent = player.total_tokens_spent.saturating_add(pack_cost);

    Ok(())
}

/// ────────────────────────────────────────────────────────────────────────────
/// SETTLE OPEN HORSE PACK (reveal phase)
/// ────────────────────────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct SettleOpenHorsePack<'info> {
    #[account(mut)]
    pub player_wallet: Signer<'info>,
    #[account(
        mut,
        constraint = player.owner == player_wallet.key() @ HorseGameError::Unauthorized,
        constraint = player.pending_action == PendingRandomAction::HorsePack @ HorseGameError::NoHorsePackPending,
        seeds = [PLAYER_SEED, player_wallet.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub player: Box<Account<'info, Player>>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        constraint = global_state.token_initialized @ HorseGameError::TokenNotInitialized,
        constraint = global_state.token_mint == token_mint.key() @ HorseGameError::InvalidTokenMint,
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        seeds = [REWARDS_VAULT_SEED, token_mint.key().as_ref()],
        bump,
    )]
    pub rewards_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = player_token_account.mint == global_state.token_mint,
        constraint = player_token_account.owner == player_wallet.key() @ HorseGameError::InvalidTokenAccountOwner
    )]
    pub player_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account()]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    /// CHECK: Checked manually
    pub slot_hashes: AccountInfo<'info>,
}

pub fn settle_open_horse_pack(ctx: Context<SettleOpenHorsePack>) -> Result<()> {
    let clock: Clock = Clock::get()?;
    let player = &mut ctx.accounts.player;
    let gs = &mut ctx.accounts.global_state;

    require!(
        clock.slot >= player.commit_slot + MIN_RANDOMNESS_DELAY_SLOTS,
        HorseGameError::RandomnessNotResolved
    );
    let reveal_slot = player.commit_slot + MIN_RANDOMNESS_DELAY_SLOTS;

    let sysvar_slot_history = &ctx.accounts.slot_hashes;
    require!(
        sysvar_slot_history.key == &slot_hashes::id(),
        HorseGameError::InvalidSlotHashes
    );

    let data = sysvar_slot_history.try_borrow_data()?;
    let num_slot_hashes = u64::from_le_bytes(data[0..8].try_into().unwrap());
    let mut pos = 8;
    let mut found_hash = None;
    for _ in 0..num_slot_hashes {
        let slot = u64::from_le_bytes(data[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let hash = &data[pos..pos + 32];
        if slot == reveal_slot {
            found_hash = Some(hash);
            break;
        }
        pos += 32;
    }

    let random_value = found_hash.ok_or(HorseGameError::SlotNotFound)?;

    settle_and_mint_rewards(
        player,
        gs,
        clock.slot,
        &ctx.accounts.player_token_account.to_account_info(),
        &ctx.accounts.token_mint.to_account_info(),
        &ctx.accounts.rewards_vault,
        &ctx.accounts.token_program.to_account_info(),
        ctx.bumps.global_state,
        ctx.accounts.token_mint.decimals,
    )?;

    let mut horse_ids = [0u16; 5];
    for i in 0..5 {
        let slice_start = i * 4;
        let slice_end = slice_start + 4;
        let mut random_bytes: [u8; 4] = [0; 4];
        random_bytes.copy_from_slice(&random_value[slice_start..slice_end]);
        let random_u32 = u32::from_le_bytes(random_bytes);

        let random_percent = (random_u32 as u64 * 1000 / (u32::MAX as u64 + 1)) as u32;

        let grade = match random_percent {
            0..=499 => GRADE_E,      // 50.0%
            500..=749 => GRADE_D,    // 25.0%
            750..=899 => GRADE_C,    // 15.0%
            900..=989 => GRADE_B,    // 9.0%
            _ => GRADE_SS,           // 1.0%
        };

        let horses_of_grade: Vec<&(u16, u8, u16, u8)> = HORSE_DATA
            .iter()
            .filter(|(_, horse_grade, _, _)| *horse_grade == grade)
            .collect();

        if !horses_of_grade.is_empty() {
            let horse_index =
                (random_u32 as u64 * horses_of_grade.len() as u64 / (u32::MAX as u64 + 1)) as usize;

            let (horse_id, _, speed, stamina_cost) = horses_of_grade[horse_index];

            require!(
                (player.horse_count as usize) < MAX_HORSES_PER_PLAYER as usize,
                HorseGameError::StableCapacityExceeded
            );

            let new_horse = Horse {
                id: *horse_id,
                grade,
                speed: *speed,
                stamina_cost: *stamina_cost,
            };
            player.add_horse(new_horse)?;
            horse_ids[i] = *horse_id;
        }
    }

    player.pending_action = PendingRandomAction::None;
    player.commit_slot = 0;
    player.total_horse_packs_opened = player.total_horse_packs_opened.saturating_add(1);
    gs.total_horse_packs_opened = gs.total_horse_packs_opened.saturating_add(1);

    emit!(HorsePackOpened {
        player: player.key(),
        horse_ids: horse_ids.map(|id| id as u8),
    });

    Ok(())
}

/// ────────────────────────────────────────────────────────────────────────────
/// ADMIN FUNCTIONS
/// ────────────────────────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct ToggleProduction<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        has_one = authority @ HorseGameError::Unauthorized
    )]
    pub global_state: Account<'info, GlobalState>,
}

pub fn toggle_production(ctx: Context<ToggleProduction>, enable: bool) -> Result<()> {
    ctx.accounts.global_state.production_enabled = enable;
    Ok(())
}

#[derive(Accounts)]
pub struct UpdateParameters<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        has_one = authority @ HorseGameError::Unauthorized
    )]
    pub global_state: Account<'info, GlobalState>,
}

pub fn update_parameter(
    ctx: Context<UpdateParameters>,
    parameter_index: u8,
    parameter_value: u64,
) -> Result<()> {
    let gs = &mut ctx.accounts.global_state;

    match parameter_index {
        0 => {
            require!(parameter_value <= 100, HorseGameError::InvalidReferralFee);
            gs.referral_fee = parameter_value as u8;
        }
        1 => {
            require!(parameter_value <= 100, HorseGameError::InvalidBurnRate);
            gs.burn_rate = parameter_value as u8;
        }
        2 => {
            require!(parameter_value > 0, HorseGameError::InvalidDustThresholdDivisor);
            gs.dust_threshold_divisor = parameter_value;
        }
        3 => gs.initial_stable_purchase_fee_lamports = parameter_value,
        4 => gs.horse_pack_cost_microtokens = parameter_value,
        5 => gs.gamble_fee_lamports = parameter_value,
        6 => gs.staking_lockup_slots = parameter_value,
        7 => gs.token_reward_rate = parameter_value,
        8 => gs.reward_rate = parameter_value,
        _ => return err!(HorseGameError::InvalidParameterIndex),
    }

    Ok(())
}

#[derive(Accounts)]
pub struct UpdatePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        bump,
        has_one = authority @ HorseGameError::Unauthorized
    )]
    pub global_state: Account<'info, GlobalState>,
}

pub fn update_pool_manual(ctx: Context<UpdatePool>) -> Result<()> {
    let slot_now: u64 = Clock::get()?.slot;
    update_pool(&mut ctx.accounts.global_state, slot_now);
    Ok(())
}

#[derive(Accounts)]
pub struct ResetPlayer<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        has_one = authority @ HorseGameError::Unauthorized,
        seeds = [GLOBAL_STATE_SEED],
        constraint = global_state.token_initialized @ HorseGameError::TokenNotInitialized,
        constraint = global_state.token_mint == token_mint.key() @ HorseGameError::InvalidTokenMint,
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        seeds = [PLAYER_SEED, player_wallet.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub player: Box<Account<'info, Player>>,
    #[account()]
    pub token_mint: InterfaceAccount<'info, Mint>,
    /// CHECK: System account
    pub player_wallet: AccountInfo<'info>,
}

pub fn reset_player(ctx: Context<ResetPlayer>) -> Result<()> {
    let player = &mut ctx.accounts.player;
    let gs = &mut ctx.accounts.global_state;
    let slot = Clock::get()?.slot;

    update_pool(gs, slot);

    let old_feed = player.feed_consumption;
    let old_speed = player.total_speed;

    player.feed_consumption = 0;
    player.total_speed = 0;
    let (racing_slots, feed_capacity, _) = STABLE_CONFIGS[0];
    player.stable = Stable {
        stable_type: 0,
        racing_slots,
        feed_capacity,
    };
    player.horses = [Horse::default(); MAX_HORSES_PER_PLAYER as usize];
    player.horse_count = 0;
    player.racing_horses_bitset = 0;

    gs.total_feed_consumption = gs.total_feed_consumption.saturating_sub(old_feed);
    gs.total_speed = gs.total_speed.saturating_sub(old_speed);

    player.last_claim_slot = slot;
    player.last_acc_tokens_per_speed = gs.acc_tokens_per_speed;
    player.pending_action = PendingRandomAction::None;
    player.commit_slot = 0;

    Ok(())
}

/// ────────────────────────────────────────────────────────────────────────────
/// BREED HORSES (commit phase)
/// ────────────────────────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct BreedHorsesCommit<'info> {
    #[account(mut)]
    pub player_wallet: Signer<'info>,
    #[account(
        mut,
        constraint = player.owner == player_wallet.key() @ HorseGameError::Unauthorized,
        constraint = player.pending_action == PendingRandomAction::None @ HorseGameError::BreedingAlreadyPending,
        seeds = [PLAYER_SEED, player_wallet.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub player: Box<Account<'info, Player>>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        constraint = global_state.token_initialized @ HorseGameError::TokenNotInitialized,
        constraint = global_state.token_mint == token_mint.key() @ HorseGameError::InvalidTokenMint,
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        seeds = [REWARDS_VAULT_SEED, token_mint.key().as_ref()],
        bump,
    )]
    pub rewards_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = player_token_account.mint == global_state.token_mint,
        constraint = player_token_account.owner == player_wallet.key() @ HorseGameError::InvalidTokenAccountOwner
    )]
    pub player_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account()]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn breed_horses_commit(ctx: Context<BreedHorsesCommit>, horse_indices: Vec<u8>) -> Result<()> {
    let slot = Clock::get()?.slot;
    let player = &mut ctx.accounts.player;
    let gs = &mut ctx.accounts.global_state;

    require!(gs.production_enabled, HorseGameError::ProductionDisabled);
    require!(
        !horse_indices.is_empty() && horse_indices.len() <= 128,
        HorseGameError::InvalidBreedingHorseCount
    );
    require!(
        player.horse_count as usize >= horse_indices.len(),
        HorseGameError::InvalidBreedingHorseCount
    );

    settle_and_mint_rewards(
        player,
        gs,
        slot,
        &ctx.accounts.player_token_account.to_account_info(),
        &ctx.accounts.token_mint.to_account_info(),
        &ctx.accounts.rewards_vault,
        &ctx.accounts.token_program.to_account_info(),
        ctx.bumps.global_state,
        ctx.accounts.token_mint.decimals,
    )?;

    let mut sorted_indices = horse_indices.clone();
    sorted_indices.sort();
    for i in 1..sorted_indices.len() {
        require!(
            sorted_indices[i] != sorted_indices[i - 1],
            HorseGameError::DuplicateBreedingHorseIndices
        );
    }
    for &index in &horse_indices {
        validate_horse_index(index, player.horse_count as usize)?;
        require!(!player.is_horse_racing(index), HorseGameError::HorseIsRacing);
    }

    let mut horse_indices_array = [0u8; 128];
    for (i, &index) in horse_indices.iter().enumerate() {
        horse_indices_array[i] = index;
    }

    player.pending_action = PendingRandomAction::Breeding {
        horse_indices: horse_indices_array,
        horse_count: horse_indices.len() as u8,
    };
    player.commit_slot = slot;

    gs.total_breeding_attempts = gs.total_breeding_attempts.saturating_add(1);

    Ok(())
}

/// ────────────────────────────────────────────────────────────────────────────
/// BREED HORSES (settle phase)
/// ────────────────────────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct BreedHorsesSettle<'info> {
    #[account(mut)]
    pub player_wallet: Signer<'info>,
    #[account(
        mut,
        constraint = player.owner == player_wallet.key() @ HorseGameError::Unauthorized,
        constraint = matches!(player.pending_action, PendingRandomAction::Breeding { .. }) @ HorseGameError::NoBreedingPending,
        seeds = [PLAYER_SEED, player_wallet.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub player: Box<Account<'info, Player>>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        constraint = global_state.token_initialized @ HorseGameError::TokenNotInitialized,
        constraint = global_state.token_mint == token_mint.key() @ HorseGameError::InvalidTokenMint,
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        seeds = [REWARDS_VAULT_SEED, token_mint.key().as_ref()],
        bump,
    )]
    pub rewards_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = player_token_account.mint == global_state.token_mint,
        constraint = player_token_account.owner == player_wallet.key() @ HorseGameError::InvalidTokenAccountOwner
    )]
    pub player_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account()]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    /// CHECK: Checked manually
    pub slot_hashes: AccountInfo<'info>,
}

pub fn breed_horses_settle(ctx: Context<BreedHorsesSettle>) -> Result<()> {
    let clock: Clock = Clock::get()?;
    let player = &mut ctx.accounts.player;
    let gs = &mut ctx.accounts.global_state;

    require!(
        clock.slot >= player.commit_slot + MIN_RANDOMNESS_DELAY_SLOTS,
        HorseGameError::RandomnessNotResolved
    );
    let reveal_slot = player.commit_slot + MIN_RANDOMNESS_DELAY_SLOTS;

    let sysvar_slot_history = &ctx.accounts.slot_hashes;
    require!(
        sysvar_slot_history.key == &slot_hashes::id(),
        HorseGameError::InvalidSlotHashes
    );

    let data = sysvar_slot_history.try_borrow_data()?;
    let num_slot_hashes = u64::from_le_bytes(data[0..8].try_into().unwrap());
    let mut pos = 8;
    let mut found_hash = None;
    for _ in 0..num_slot_hashes {
        let slot = u64::from_le_bytes(data[pos..pos + 8].try_into().unwrap());
        pos += 8;
        let hash = &data[pos..pos + 32];
        if slot == reveal_slot {
            found_hash = Some(hash);
            break;
        }
        pos += 32;
    }

    let random_value = found_hash.ok_or(HorseGameError::SlotNotFound)?;

    settle_and_mint_rewards(
        player,
        gs,
        clock.slot,
        &ctx.accounts.player_token_account.to_account_info(),
        &ctx.accounts.token_mint.to_account_info(),
        &ctx.accounts.rewards_vault,
        &ctx.accounts.token_program.to_account_info(),
        ctx.bumps.global_state,
        ctx.accounts.token_mint.decimals,
    )?;

    let (horse_indices_array, horse_count) = if let PendingRandomAction::Breeding {
        horse_indices,
        horse_count,
    } = player.pending_action
    {
        (horse_indices, horse_count)
    } else {
        return Err(HorseGameError::NoBreedingPending.into());
    };

    let mut successful_offspring = 0u8;
    let mut new_horses: Vec<(u16, u8, u16, u8)> = Vec::new();

    for i in 0..horse_count {
        let horse_index = horse_indices_array[i as usize];

        if (horse_index as usize) >= (player.horse_count as usize) {
            continue;
        }

        let horse = &player.horses[horse_index as usize];
        let current_grade = horse.grade;

        let random_byte_index = (i as usize) % random_value.len();
        let random_byte = random_value[random_byte_index];

        // 20% chance to get offspring of next grade
        if random_byte < 51 {
            if let Some(next_grade) = get_next_grade(current_grade) {
                let horses_of_next_grade: Vec<&(u16, u8, u16, u8)> = HORSE_DATA
                    .iter()
                    .filter(|(_, horse_grade, _, _)| *horse_grade == next_grade)
                    .collect();

                if !horses_of_next_grade.is_empty() {
                    let mut random_bytes: [u8; 4] = [0; 4];
                    let start_idx = (i as usize * 4) % (random_value.len() - 3);
                    random_bytes.copy_from_slice(&random_value[start_idx..start_idx + 4]);
                    let random_u32 = u32::from_le_bytes(random_bytes);

                    let horse_index_in_grade = (random_u32 as usize) % horses_of_next_grade.len();
                    let (horse_id, _, speed, stamina_cost) =
                        horses_of_next_grade[horse_index_in_grade];

                    new_horses.push((*horse_id, next_grade, *speed, *stamina_cost));
                    successful_offspring += 1;
                }
            }
        }
    }

    let indices_to_remove: Vec<u8> = horse_indices_array[0..horse_count as usize].to_vec();
    player.batch_remove_horses(&indices_to_remove)?;

    for (horse_id, grade, speed, stamina_cost) in new_horses {
        require!(
            (player.horse_count as usize) < MAX_HORSES_PER_PLAYER as usize,
            HorseGameError::StableCapacityExceeded
        );

        let new_horse = Horse {
            id: horse_id,
            grade,
            speed,
            stamina_cost,
        };
        player.add_horse(new_horse)?;
    }

    player.pending_action = PendingRandomAction::None;
    player.commit_slot = 0;

    player.total_horses_bred = player
        .total_horses_bred
        .saturating_add(horse_count as u64);

    if successful_offspring > 0 {
        player.successful_breeding = player
            .successful_breeding
            .saturating_add(successful_offspring as u64);
        gs.total_successful_breeding = gs
            .total_successful_breeding
            .saturating_add(successful_offspring as u64);
    }

    emit!(HorsesBred {
        player: player.key(),
        successful_offspring,
        total_bred: horse_count,
    });

    Ok(())
}

/// ────────────────────────────────────────────────────────────────────────────
/// CANCEL PENDING ACTION
/// ────────────────────────────────────────────────────────────────────────────
#[derive(Accounts)]
pub struct CancelPendingAction<'info> {
    #[account(mut)]
    pub player_wallet: Signer<'info>,
    #[account(
        mut,
        constraint = player.owner == player_wallet.key() @ HorseGameError::Unauthorized,
        constraint = player.pending_action != PendingRandomAction::None @ HorseGameError::NoPendingAction,
        seeds = [PLAYER_SEED, player_wallet.key().as_ref(), token_mint.key().as_ref()],
        bump
    )]
    pub player: Box<Account<'info, Player>>,
    #[account(
        mut,
        seeds = [GLOBAL_STATE_SEED],
        constraint = global_state.token_initialized @ HorseGameError::TokenNotInitialized,
        constraint = global_state.token_mint == token_mint.key() @ HorseGameError::InvalidTokenMint,
        bump,
    )]
    pub global_state: Account<'info, GlobalState>,
    #[account(
        mut,
        seeds = [REWARDS_VAULT_SEED, token_mint.key().as_ref()],
        bump,
    )]
    pub rewards_vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        constraint = player_token_account.mint == global_state.token_mint,
        constraint = player_token_account.owner == player_wallet.key() @ HorseGameError::InvalidTokenAccountOwner
    )]
    pub player_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account()]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn cancel_pending_action(ctx: Context<CancelPendingAction>) -> Result<()> {
    let player = &mut ctx.accounts.player;
    let clock = Clock::get()?;

    settle_and_mint_rewards(
        player,
        &mut ctx.accounts.global_state,
        clock.slot,
        &ctx.accounts.player_token_account.to_account_info(),
        &ctx.accounts.token_mint.to_account_info(),
        &ctx.accounts.rewards_vault,
        &ctx.accounts.token_program.to_account_info(),
        ctx.bumps.global_state,
        ctx.accounts.token_mint.decimals,
    )?;

    require!(
        clock.slot > player.commit_slot + CANCEL_TIMEOUT_SLOTS,
        HorseGameError::CancelTimeoutNotExpired
    );

    if let PendingRandomAction::Breeding {
        horse_indices,
        horse_count,
    } = player.pending_action.clone()
    {
        let mut indices_to_remove: Vec<u8> = horse_indices[0..horse_count as usize].to_vec();
        indices_to_remove.sort_by(|a, b| b.cmp(a));
        player.batch_remove_horses(&indices_to_remove)?;
    }

    player.pending_action = PendingRandomAction::None;
    player.commit_slot = 0;

    Ok(())
}

/// Helper function to handle fee transfers with referral logic
fn handle_fee_transfers<'info>(
    player: &mut Box<Account<'info, Player>>,
    gs: &mut Account<'info, GlobalState>,
    total_amount: u64,
    player_token_account: &AccountInfo<'info>,
    fees_token_account: &AccountInfo<'info>,
    referrer_token_account: Option<InterfaceAccount<'info, TokenAccount>>,
    player_wallet: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    token_mint: &AccountInfo<'info>,
    is_horse_pack: bool,
    decimals: u8,
) -> Result<()> {
    let burn_amount = total_amount
        .saturating_mul(gs.burn_rate as u64)
        .saturating_div(100);
    let fees_amount = total_amount.saturating_sub(burn_amount);

    if burn_amount > 0 {
        gs.burned_tokens = gs.burned_tokens.saturating_add(burn_amount);
        token_interface::burn(
            CpiContext::new(
                token_program.clone(),
                Burn {
                    mint: token_mint.clone(),
                    from: player_token_account.clone(),
                    authority: player_wallet.clone(),
                },
            ),
            burn_amount,
        )?;
    }

    if is_horse_pack {
        if referrer_token_account.is_some() {
            let referral_commission = fees_amount
                .saturating_mul(gs.referral_fee as u64)
                .saturating_div(100);
            let protocol_fee = fees_amount.saturating_sub(referral_commission);

            if referral_commission > 0 {
                token_interface::transfer_checked(
                    CpiContext::new(
                        token_program.clone(),
                        token_interface::TransferChecked {
                            from: player_token_account.clone(),
                            mint: token_mint.clone(),
                            to: referrer_token_account.clone().unwrap().to_account_info(),
                            authority: player_wallet.clone(),
                        },
                    ),
                    referral_commission,
                    decimals,
                )?;
                player.total_earnings_for_referrer = player
                    .total_earnings_for_referrer
                    .saturating_add(referral_commission);
            }

            if protocol_fee > 0 {
                token_interface::transfer_checked(
                    CpiContext::new(
                        token_program.clone(),
                        token_interface::TransferChecked {
                            from: player_token_account.clone(),
                            mint: token_mint.clone(),
                            to: fees_token_account.clone(),
                            authority: player_wallet.clone(),
                        },
                    ),
                    protocol_fee,
                    decimals,
                )?;
            }
            return Ok(());
        }
    }

    if fees_amount > 0 {
        token_interface::transfer_checked(
            CpiContext::new(
                token_program.clone(),
                token_interface::TransferChecked {
                    from: player_token_account.clone(),
                    mint: token_mint.clone(),
                    to: fees_token_account.clone(),
                    authority: player_wallet.clone(),
                },
            ),
            fees_amount,
            decimals,
        )?;
    }

    Ok(())
}

