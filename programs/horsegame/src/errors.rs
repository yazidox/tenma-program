use anchor_lang::prelude::*;

#[error_code]
pub enum HorseGameError {
    #[msg("Unauthorized access")]
    Unauthorized,

    #[msg("Production is disabled")]
    ProductionDisabled,

    #[msg("Initial stable already purchased")]
    InitialStableAlreadyPurchased,

    #[msg("Self referral not allowed")]
    SelfReferralNotAllowed,

    #[msg("Stable capacity exceeded")]
    StableCapacityExceeded,

    #[msg("Feed capacity exceeded")]
    FeedCapacityExceeded,

    #[msg("Horse index out of bounds")]
    HorseIndexOutOfBounds,

    #[msg("Horse is already racing")]
    HorseIsRacing,

    #[msg("Horse is not racing")]
    HorseNotRacing,

    #[msg("Horse is pending breeding")]
    HorsePendingBreeding,

    #[msg("Invalid stable type")]
    InvalidStableType,

    #[msg("Insufficient tokens")]
    InsufficientTokens,

    #[msg("Cooldown not expired")]
    CooldownNotExpired,

    #[msg("Horse pack already pending")]
    HorsePackAlreadyPending,

    #[msg("No horse pack pending")]
    NoHorsePackPending,

    #[msg("Randomness not resolved")]
    RandomnessNotResolved,

    #[msg("Invalid slot hashes")]
    InvalidSlotHashes,

    #[msg("Slot not found")]
    SlotNotFound,

    #[msg("Invalid token account owner")]
    InvalidTokenAccountOwner,

    #[msg("Invalid mint authority")]
    InvalidMintAuthority,

    #[msg("Invalid decimals")]
    InvalidDecimals,

    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,

    #[msg("Invalid referral fee")]
    InvalidReferralFee,

    #[msg("Invalid burn rate")]
    InvalidBurnRate,

    #[msg("Invalid dust threshold divisor")]
    InvalidDustThresholdDivisor,

    #[msg("Invalid parameter index")]
    InvalidParameterIndex,

    #[msg("Invalid breeding horse count")]
    InvalidBreedingHorseCount,

    #[msg("Duplicate breeding horse indices")]
    DuplicateBreedingHorseIndices,

    #[msg("Breeding already pending")]
    BreedingAlreadyPending,

    #[msg("No breeding pending")]
    NoBreedingPending,

    #[msg("No pending action")]
    NoPendingAction,

    #[msg("Cancel timeout not expired")]
    CancelTimeoutNotExpired,

    #[msg("Referrer account missing")]
    ReferrerAccountMissing,

    #[msg("Token mint not initialized - call set_token_mint first")]
    TokenNotInitialized,

    #[msg("Invalid token mint - does not match configured token")]
    InvalidTokenMint,
}

