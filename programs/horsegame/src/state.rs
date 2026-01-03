use crate::constants::*;
use crate::HorseGameError;
use anchor_lang::prelude::*;

#[account]
pub struct GlobalState {
    /* ── governance ─────────────────────────────── */
    pub authority: Pubkey,   // Governance authority
    pub token_mint: Pubkey,  // Token mint (Pubkey::default() if not set)
    pub fees_wallet: Pubkey, // Wallet that receives SOL and token fees
    pub rewards_vault: Pubkey, // Rewards vault (Pubkey::default() if not set)
    pub token_initialized: bool, // Whether token mint has been set

    /* ── emission mechanics ─────────────────────── */
    pub total_supply: u64,              // Hard cap (mint-burn accounting)
    pub burned_tokens: u64,             // Total tokens destroyed with `token::burn`
    pub cumulative_rewards: u64,        // Total tokens ever minted as rewards
    pub start_slot: u64,                // Genesis slot
    pub reward_rate: u64,               // Reward per slot
    pub acc_tokens_per_speed: u128,     // 1e12-scaled accumulator (rewards per speed point)
    pub last_reward_slot: u64,          // When `acc_tokens_per_speed` was last bumped

    /* ── economic params ────────────────────────── */
    pub burn_rate: u8,               // % of token cost burned (default 80)
    pub referral_fee: u8,            // % of fees to referrer (default 100 = all non-burned fees)
    pub production_enabled: bool,    // Global kill-switch
    pub dust_threshold_divisor: u64, // Divisor for total_supply to get dust_threshold

    /* ── fee configuration ──────────────────────── */
    pub initial_stable_purchase_fee_lamports: u64, // SOL cost to start playing
    pub horse_pack_cost_microtokens: u64,          // Token cost for horse pack
    pub gamble_fee_lamports: u64,                  // SOL fee for gambling

    /* ── gameplay stats ─────────────────────────── */
    pub total_feed_consumption: u64, // Σ player feed consumption (stamina tracking)
    pub total_speed: u64,            // Σ racing horses' speed (for reward distribution)

    /* ── gambling stats ───────────────────────── */
    pub total_global_gambles: u64,
    pub total_global_gamble_wins: u64,

    /* ── horse pack stats ──────────────────────── */
    pub total_horse_packs_opened: u64,
    pub total_breeding_attempts: u64,
    pub total_successful_breeding: u64,

    /* ── staking pool ───────────────────────────── */
    pub total_staked_tokens: u64,
    pub staking_lockup_slots: u64,
    pub acc_sol_rewards_per_token: u128,
    pub acc_token_rewards_per_token: u128,
    pub last_staking_reward_slot: u64,
    pub token_reward_rate: u64,
    pub total_sol_deposited: u64,

    /* ── future expansion ───────────────────────── */
    pub padding: [u8; 64],
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum PendingRandomAction {
    None,
    Gamble {
        amount: u64,
    },
    HorsePack,
    Breeding {
        horse_indices: [u8; 128],
        horse_count: u8,
    },
}

impl Default for PendingRandomAction {
    fn default() -> Self {
        PendingRandomAction::None
    }
}

#[account]
pub struct Player {
    pub owner: Pubkey,
    pub stable: Stable,
    pub horses: [Horse; MAX_HORSES_PER_PLAYER as usize],
    pub horse_count: u8,
    pub racing_horses_bitset: u128,  // Tracks which horses are racing
    pub feed_consumption: u64,       // Total feed needed by racing horses
    pub total_speed: u64,            // Total speed of racing horses
    pub referrer: Option<Pubkey>,
    pub last_acc_tokens_per_speed: u128,
    pub last_claim_slot: u64,
    pub last_upgrade_slot: u64,
    pub total_rewards: u64,
    pub total_gambles: u64,
    pub total_gamble_wins: u64,

    pub pending_action: PendingRandomAction,
    pub commit_slot: u64,

    /* ── additional player stats ──────────────────── */
    pub total_earnings_for_referrer: u64,
    pub total_horse_packs_opened: u64,
    pub total_horses_bred: u64,
    pub successful_breeding: u64,
    pub total_sol_spent: u64,
    pub total_tokens_spent: u64,

    /* ── staking stats ──────────────────────────── */
    pub staked_tokens: u64,
    pub last_stake_slot: u64,
    pub last_acc_sol_rewards_per_token: u128,
    pub last_acc_token_rewards_per_token: u128,
    pub claimed_token_rewards: u64,

    /* ── racing stats ──────────────────────────── */
    pub total_races_entered: u64,
    pub total_race_wins: u64,

    /* ── future expansion ───────────────────────── */
    pub padding: [u8; 64],
}

impl Player {
    pub fn add_horse(&mut self, horse: Horse) -> Result<()> {
        require!(
            (self.horse_count as usize) < MAX_HORSES_PER_PLAYER as usize,
            HorseGameError::StableCapacityExceeded
        );
        self.horses[self.horse_count as usize] = horse;
        self.horse_count += 1;
        Ok(())
    }

    pub fn batch_remove_horses(&mut self, indices: &[u8]) -> Result<()> {
        let mut new_horses = Vec::with_capacity(self.horse_count as usize);
        let mut new_bitset = 0u128;
        let mut current_index = 0;

        for i in 0..self.horse_count {
            if !indices.contains(&i) {
                new_horses.push(self.horses[i as usize]);
                if self.racing_horses_bitset & (1u128 << i) != 0 {
                    new_bitset |= 1u128 << current_index;
                }
                current_index += 1;
            }
        }

        for i in 0..new_horses.len() {
            self.horses[i] = new_horses[i];
        }
        for i in new_horses.len()..self.horse_count as usize {
            self.horses[i] = Horse::default();
        }
        self.horse_count = new_horses.len() as u8;
        self.racing_horses_bitset = new_bitset;

        Ok(())
    }

    pub fn enter_horse_in_race(&mut self, index: u8) -> Result<()> {
        require!(index < 128, HorseGameError::HorseIndexOutOfBounds);
        let mask = 1u128 << index;
        require!(
            self.racing_horses_bitset & mask == 0,
            HorseGameError::HorseIsRacing
        );
        self.racing_horses_bitset |= mask;
        Ok(())
    }

    pub fn withdraw_horse_from_race(&mut self, index: u8) -> Result<()> {
        require!(index < 128, HorseGameError::HorseIndexOutOfBounds);
        let mask = 1u128 << index;
        require!(
            self.racing_horses_bitset & mask != 0,
            HorseGameError::HorseNotRacing
        );
        self.racing_horses_bitset &= !mask;
        Ok(())
    }

    pub fn is_horse_racing(&self, index: u8) -> bool {
        if index >= 128 {
            return false;
        }
        (self.racing_horses_bitset & (1u128 << index)) != 0
    }

    pub fn count_racing_horses(&self) -> u8 {
        self.racing_horses_bitset.count_ones() as u8
    }

    pub fn calculate_total_feed_consumption(&self) -> u64 {
        let mut total = 0u64;
        for i in 0..self.horse_count {
            if self.is_horse_racing(i) {
                let horse = &self.horses[i as usize];
                total += horse.stamina_cost as u64;
            }
        }
        total
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Stable {
    pub stable_type: u8,
    pub racing_slots: u8,    // Max number of horses that can race
    pub feed_capacity: u64,  // Total feed capacity
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default)]
pub struct Horse {
    pub id: u16,            // Horse ID
    pub grade: u8,          // Grade (E=0, D=1, C=2, B=3, SS=4)
    pub speed: u16,         // Speed rating for racing rewards
    pub stamina_cost: u8,   // Feed consumption per slot
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct SpeedCheckpoint {
    pub slot: u64,
    pub total_speed: u64,
    pub accumulated_rewards: u64,
}

