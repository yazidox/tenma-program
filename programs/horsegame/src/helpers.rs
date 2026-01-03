use crate::{constants::*, errors::HorseGameError};
use anchor_lang::prelude::*;

/// Validates that a horse index is within bounds
pub fn validate_horse_index(horse_index: u8, horses_len: usize) -> Result<()> {
    require!(
        (horse_index as usize) < horses_len,
        HorseGameError::HorseIndexOutOfBounds
    );
    Ok(())
}

/// Safely adds feed consumption, checking for overflow
pub fn safe_add_feed(current: u64, to_add: u64) -> Result<u64> {
    current
        .checked_add(to_add)
        .ok_or(HorseGameError::ArithmeticOverflow.into())
}

/// Safely subtracts feed consumption, checking for underflow
pub fn safe_sub_feed(current: u64, to_sub: u64) -> Result<u64> {
    current
        .checked_sub(to_sub)
        .ok_or(HorseGameError::ArithmeticOverflow.into())
}

/// Safely adds speed, checking for overflow
pub fn safe_add_speed(current: u64, to_add: u64) -> Result<u64> {
    current
        .checked_add(to_add)
        .ok_or(HorseGameError::ArithmeticOverflow.into())
}

/// Safely subtracts speed, checking for underflow
pub fn safe_sub_speed(current: u64, to_sub: u64) -> Result<u64> {
    current
        .checked_sub(to_sub)
        .ok_or(HorseGameError::ArithmeticOverflow.into())
}

/// Gets the next higher grade for breeding
pub fn get_next_grade(current_grade: u8) -> Option<u8> {
    match current_grade {
        GRADE_E => Some(GRADE_D),
        GRADE_D => Some(GRADE_C),
        GRADE_C => Some(GRADE_B),
        GRADE_B => Some(GRADE_SS),
        GRADE_SS => Some(GRADE_SS), // Already at max grade
        _ => None,
    }
}

/// Calculates the current reward rate based on elapsed time since start_slot
pub fn calculate_current_reward_rate(current_slot: u64, start_slot: u64) -> u64 {
    if current_slot < start_slot {
        return 0;
    }

    let elapsed_slots = current_slot.saturating_sub(start_slot);

    if elapsed_slots < STAGE_1_DURATION_SLOTS {
        STAGE_1_REWARD_RATE
    } else if elapsed_slots < STAGE_1_DURATION_SLOTS + STAGE_2_DURATION_SLOTS {
        STAGE_2_REWARD_RATE
    } else if elapsed_slots
        < STAGE_1_DURATION_SLOTS + STAGE_2_DURATION_SLOTS + STAGE_3_DURATION_SLOTS
    {
        STAGE_3_REWARD_RATE
    } else {
        0 // After 90 days, no more rewards
    }
}

pub fn calculate_halvings(current_slot: u64, start_slot: u64, halving_interval: u64) -> u64 {
    current_slot.saturating_sub(start_slot) / halving_interval
}

pub fn calculate_max_halvings(initial_reward_rate: u64) -> u64 {
    if initial_reward_rate == 0 {
        return 0;
    }
    64 - initial_reward_rate.leading_zeros() as u64
}

pub fn reward_after_halvings(initial: u64, halvings: u64) -> u64 {
    initial.checked_shr(halvings as u32).unwrap_or(0)
}

