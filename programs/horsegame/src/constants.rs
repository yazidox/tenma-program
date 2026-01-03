pub const GLOBAL_STATE_SEED: &[u8] = b"global_state";
pub const PLAYER_SEED: &[u8] = b"player";
pub const STAKING_VAULT_SEED: &[u8] = b"staking_vault";
pub const SOL_REWARDS_WALLET_SEED: &[u8] = b"sol_rewards_wallet";
pub const REWARDS_VAULT_SEED: &[u8] = b"rewards_vault";

// Fixed variables
pub const ACC_SCALE: u128 = 1_000_000_000_000; // 1e12

// Security constants
pub const MIN_RANDOMNESS_DELAY_SLOTS: u64 = 2;
pub const MAX_HORSES_PER_PLAYER: u8 = 128;
pub const MAX_RACING_HORSES_PER_PLAYER: u8 = 25;
pub const CANCEL_TIMEOUT_SLOTS: u64 = 24; // Approx. 60 seconds

// Horse Breeds/Tiers (rarity equivalent)
pub const GRADE_E: u8 = 0;      // Common - Draft horses, ponies
pub const GRADE_D: u8 = 1;      // Uncommon - Quarter horses
pub const GRADE_C: u8 = 2;      // Rare - Standardbreds
pub const GRADE_B: u8 = 3;      // Double Rare - Hanoverians, Arabians
pub const GRADE_SS: u8 = 4;     // Legendary - Thoroughbreds, Champions

// Initial starter horse IDs
pub const STARTER_HORSE_IDS: [u16; 3] = [179, 175, 147]; // Starter horses

// === Stable configurations ===
// format: (racing_slots, feed_capacity, cost_in_microtokens)
pub const STABLE_CONFIGS: [(u8, u64, u64); 11] = [
    (0, 0, 0),                      // Level 0 - Initial state before buying first stable
    (2, 6, 0),                      // Level 1 - Small barn
    (4, 12, 1_000_000_000),         // Level 2 - Medium barn
    (7, 20, 2_000_000_000),         // Level 3 - Large barn
    (10, 40, 4_000_000_000),        // Level 4 - Ranch
    (13, 70, 8_000_000_000),        // Level 5 - Large ranch
    (16, 110, 16_000_000_000),      // Level 6 - Training facility
    (19, 230, 32_000_000_000),      // Level 7 - Professional stable
    (22, 420, 64_000_000_000),      // Level 8 - Elite stable
    (24, 800, 128_000_000_000),     // Level 9 - Championship facility
    (25, 2000, 256_000_000_000),    // Level 10 - Legendary ranch
];

// === Horse data ===
// format: (id, grade, speed, stamina_cost)
// Speed = racing performance (like hashpower)
// Stamina = feed consumption
pub const HORSE_DATA: [(u16, u8, u16, u8); 191] = [
    // Grade SS - Legendary Champions (9 horses - includes former S grade)
    (1, GRADE_SS, 2916, 128),   // Thunderbolt
    (2, GRADE_SS, 2916, 128),   // Stormchaser
    (3, GRADE_SS, 2916, 128),   // Midnight Legend
    (4, GRADE_SS, 972, 64),     // Secretariat Jr
    (5, GRADE_SS, 972, 64),     // Eclipse Runner
    (6, GRADE_SS, 972, 64),     // Golden Streak
    (7, GRADE_SS, 972, 64),     // Silver Arrow
    (8, GRADE_SS, 972, 64),     // Diamond Dust
    (9, GRADE_SS, 972, 64),     // Royal Thunder

    // Grade B - Double Rare (32 horses - includes former A grade)
    (10, GRADE_B, 324, 32),     // Desert Wind
    (11, GRADE_B, 324, 32),     // Sandstorm
    (12, GRADE_B, 324, 32),     // Oasis Dream
    (13, GRADE_B, 324, 32),     // Sahara Star
    (14, GRADE_B, 324, 32),     // Mirage
    (15, GRADE_B, 324, 32),     // Dune Dancer
    (16, GRADE_B, 324, 32),     // Phoenix Fire
    (17, GRADE_B, 324, 32),     // Sultan's Pride
    (18, GRADE_B, 324, 32),     // Bedouin Spirit
    (19, GRADE_B, 324, 32),     // Crescent Moon
    (20, GRADE_B, 324, 32),     // Pyramid Runner
    (21, GRADE_B, 324, 32),     // Nile Wind
    (22, GRADE_B, 108, 16),     // Olympic Dream
    (23, GRADE_B, 108, 16),     // Grand Prix
    (24, GRADE_B, 108, 16),     // Dressage King
    (25, GRADE_B, 108, 16),     // Show Jumper
    (26, GRADE_B, 108, 16),     // Eventing Star
    (27, GRADE_B, 108, 16),     // Cross Country
    (28, GRADE_B, 108, 16),     // Stadium Light
    (29, GRADE_B, 108, 16),     // Elegance
    (30, GRADE_B, 108, 16),     // Precision
    (31, GRADE_B, 108, 16),     // Harmony
    (32, GRADE_B, 108, 16),     // Balance
    (33, GRADE_B, 108, 16),     // Rhythm
    (34, GRADE_B, 108, 16),     // Grace
    (35, GRADE_B, 108, 16),     // Tempo
    (36, GRADE_B, 108, 16),     // Cadence
    (37, GRADE_B, 108, 16),     // Finesse
    (38, GRADE_B, 108, 16),     // Poise
    (39, GRADE_B, 108, 16),     // Composure
    (40, GRADE_B, 108, 16),     // Serenity
    (41, GRADE_B, 108, 16),     // Majesty

    // Grade C - Rare (30 horses)
    (42, GRADE_C, 36, 8),       // Trotter
    (43, GRADE_C, 36, 8),       // Pacer
    (44, GRADE_C, 36, 8),       // Harness Hero
    (45, GRADE_C, 36, 8),       // Sulky Star
    (46, GRADE_C, 36, 8),       // Track Master
    (47, GRADE_C, 36, 8),       // Circuit Runner
    (48, GRADE_C, 36, 8),       // Oval King
    (49, GRADE_C, 36, 8),       // Mile Champ
    (50, GRADE_C, 36, 8),       // Sprint Flash
    (51, GRADE_C, 36, 8),       // Distance Pro
    (52, GRADE_C, 36, 8),       // Endurance
    (53, GRADE_C, 36, 8),       // Stamina
    (54, GRADE_C, 36, 8),       // Persistence
    (55, GRADE_C, 36, 8),       // Determination
    (56, GRADE_C, 36, 8),       // Willpower
    (57, GRADE_C, 36, 8),       // Tenacity
    (58, GRADE_C, 36, 8),       // Grit
    (59, GRADE_C, 36, 8),       // Heart
    (60, GRADE_C, 36, 8),       // Soul
    (61, GRADE_C, 36, 8),       // Spirit
    (62, GRADE_C, 36, 8),       // Drive
    (63, GRADE_C, 36, 8),       // Ambition
    (64, GRADE_C, 36, 8),       // Passion
    (65, GRADE_C, 36, 8),       // Fire
    (66, GRADE_C, 36, 8),       // Blaze
    (67, GRADE_C, 36, 8),       // Flame
    (68, GRADE_C, 36, 8),       // Spark
    (69, GRADE_C, 36, 8),       // Flash
    (70, GRADE_C, 36, 8),       // Bolt
    (71, GRADE_C, 36, 8),       // Lightning

    // Grade D - Uncommon (60 horses)
    (72, GRADE_D, 12, 4),       // Barrel Racer
    (73, GRADE_D, 12, 4),       // Rodeo Star
    (74, GRADE_D, 12, 4),       // Western Wind
    (75, GRADE_D, 12, 4),       // Ranch Hand
    (76, GRADE_D, 12, 4),       // Cowboy Dream
    (77, GRADE_D, 12, 4),       // Prairie Runner
    (78, GRADE_D, 12, 4),       // Dusty Trail
    (79, GRADE_D, 12, 4),       // Sunset Rider
    (80, GRADE_D, 12, 4),       // Canyon Echo
    (81, GRADE_D, 12, 4),       // Mesa Spirit
    (82, GRADE_D, 12, 4),       // Valley Star
    (83, GRADE_D, 12, 4),       // Mountain Breeze
    (84, GRADE_D, 12, 4),       // River Stone
    (85, GRADE_D, 12, 4),       // Creek Runner
    (86, GRADE_D, 12, 4),       // Forest Path
    (87, GRADE_D, 12, 4),       // Meadow Dance
    (88, GRADE_D, 12, 4),       // Field Day
    (89, GRADE_D, 12, 4),       // Pasture Pride
    (90, GRADE_D, 12, 4),       // Barn Star
    (91, GRADE_D, 12, 4),       // Stable Mate
    (92, GRADE_D, 12, 4),       // Corral King
    (93, GRADE_D, 12, 4),       // Fence Jumper
    (94, GRADE_D, 12, 4),       // Gate Opener
    (95, GRADE_D, 12, 4),       // Trail Blazer
    (96, GRADE_D, 12, 4),       // Path Finder
    (97, GRADE_D, 12, 4),       // Way Maker
    (98, GRADE_D, 12, 4),       // Road Runner
    (99, GRADE_D, 12, 4),       // Highway Star
    (100, GRADE_D, 12, 4),      // Country Mile
    (101, GRADE_D, 12, 4),      // Farm Fresh
    (102, GRADE_D, 12, 4),      // Harvest Moon
    (103, GRADE_D, 12, 4),      // Autumn Gold
    (104, GRADE_D, 12, 4),      // Spring Step
    (105, GRADE_D, 12, 4),      // Summer Heat
    (106, GRADE_D, 12, 4),      // Winter Coat
    (107, GRADE_D, 12, 4),      // Season Change
    (108, GRADE_D, 12, 4),      // Weather Vane
    (109, GRADE_D, 12, 4),      // Wind Chime
    (110, GRADE_D, 12, 4),      // Rain Dance
    (111, GRADE_D, 12, 4),      // Storm Chaser
    (112, GRADE_D, 12, 4),      // Cloud Nine
    (113, GRADE_D, 12, 4),      // Sky High
    (114, GRADE_D, 12, 4),      // Star Gazer
    (115, GRADE_D, 12, 4),      // Moon Walker
    (116, GRADE_D, 12, 4),      // Sun Seeker
    (117, GRADE_D, 12, 4),      // Dawn Patrol
    (118, GRADE_D, 12, 4),      // Dusk Runner
    (119, GRADE_D, 12, 4),      // Twilight
    (120, GRADE_D, 12, 4),      // Midnight
    (121, GRADE_D, 12, 4),      // Daybreak
    (122, GRADE_D, 12, 4),      // Sunrise
    (123, GRADE_D, 12, 4),      // Golden Hour
    (124, GRADE_D, 12, 4),      // Blue Hour
    (125, GRADE_D, 12, 4),      // First Light
    (126, GRADE_D, 12, 4),      // Last Call
    (127, GRADE_D, 12, 4),      // Final Stretch
    (128, GRADE_D, 12, 4),      // Home Run
    (129, GRADE_D, 12, 4),      // Victory Lap
    (130, GRADE_D, 12, 4),      // Winner Circle
    (131, GRADE_D, 12, 4),      // Podium

    // Grade E - Common (60 horses)
    (132, GRADE_E, 4, 2),       // Clydesdale Jr
    (133, GRADE_E, 4, 2),       // Shire Pony
    (134, GRADE_E, 4, 2),       // Belgian Blue
    (135, GRADE_E, 4, 2),       // Percheron Pal
    (136, GRADE_E, 4, 2),       // Suffolk Punch
    (137, GRADE_E, 4, 2),       // Haflinger
    (138, GRADE_E, 4, 2),       // Fjord Friend
    (139, GRADE_E, 4, 2),       // Welsh Pony
    (140, GRADE_E, 4, 2),       // Shetland Star
    (141, GRADE_E, 4, 2),       // Connemara
    (142, GRADE_E, 4, 2),       // Dartmoor
    (143, GRADE_E, 4, 2),       // Exmoor
    (144, GRADE_E, 4, 2),       // Fell Pony
    (145, GRADE_E, 4, 2),       // Highland
    (146, GRADE_E, 4, 2),       // New Forest
    (147, GRADE_E, 4, 2),       // Lucky Clover
    (148, GRADE_E, 4, 2),       // Gentle Giant
    (149, GRADE_E, 4, 2),       // Steady Eddie
    (150, GRADE_E, 4, 2),       // Reliable
    (151, GRADE_E, 4, 2),       // Trusty
    (152, GRADE_E, 4, 2),       // Faithful
    (153, GRADE_E, 4, 2),       // Loyal
    (154, GRADE_E, 4, 2),       // Devoted
    (155, GRADE_E, 4, 2),       // Dedicated
    (156, GRADE_E, 4, 2),       // Committed
    (157, GRADE_E, 4, 2),       // Steadfast
    (158, GRADE_E, 4, 2),       // Resolute
    (159, GRADE_E, 4, 2),       // Determined
    (160, GRADE_E, 4, 2),       // Focused
    (161, GRADE_E, 4, 2),       // Driven
    (162, GRADE_E, 4, 2),       // Motivated
    (163, GRADE_E, 4, 2),       // Inspired
    (164, GRADE_E, 4, 2),       // Eager
    (165, GRADE_E, 4, 2),       // Keen
    (166, GRADE_E, 4, 2),       // Ready
    (167, GRADE_E, 4, 2),       // Willing
    (168, GRADE_E, 4, 2),       // Able
    (169, GRADE_E, 4, 2),       // Capable
    (170, GRADE_E, 4, 2),       // Competent
    (171, GRADE_E, 4, 2),       // Skilled
    (172, GRADE_E, 4, 2),       // Talented
    (173, GRADE_E, 4, 2),       // Gifted
    (174, GRADE_E, 4, 2),       // Blessed
    (175, GRADE_E, 4, 2),       // Lucky Star
    (176, GRADE_E, 4, 2),       // Fortune
    (177, GRADE_E, 4, 2),       // Destiny
    (178, GRADE_E, 4, 2),       // Fate
    (179, GRADE_E, 4, 2),       // Chance
    (180, GRADE_E, 4, 2),       // Hope
    (181, GRADE_E, 4, 2),       // Dream
    (182, GRADE_E, 4, 2),       // Wish
    (183, GRADE_E, 4, 2),       // Wonder
    (184, GRADE_E, 4, 2),       // Magic
    (185, GRADE_E, 4, 2),       // Miracle
    (186, GRADE_E, 4, 2),       // Blessing
    (187, GRADE_E, 4, 2),       // Grace
    (188, GRADE_E, 4, 2),       // Mercy
    (189, GRADE_E, 4, 2),       // Peace
    (190, GRADE_E, 4, 2),       // Joy
    (191, GRADE_E, 4, 2),       // Love
];

// === Multi-stage reward system constants ===
// Each slot represents 400ms, so:
// 1 day = 86,400 seconds / 0.4 seconds per slot = 216,000 slots per day
pub const SLOTS_PER_DAY: u64 = 216_000;

// Three stages of 30 days each
pub const STAGE_1_DURATION_SLOTS: u64 = 30 * SLOTS_PER_DAY; // Days 1-30
pub const STAGE_2_DURATION_SLOTS: u64 = 30 * SLOTS_PER_DAY; // Days 31-60
pub const STAGE_3_DURATION_SLOTS: u64 = 30 * SLOTS_PER_DAY; // Days 61-90

// Reward rates in micro-tokens per slot (assuming 6 decimals)
pub const STAGE_1_REWARD_RATE: u64 = 1_180_556;
pub const STAGE_2_REWARD_RATE: u64 = 918_208;
pub const STAGE_3_REWARD_RATE: u64 = 655_847;

// Helper function to get horse data by ID
pub fn get_horse_by_id(id: u16) -> Option<(u8, u16, u8)> {
    HORSE_DATA
        .iter()
        .find(|(horse_id, _, _, _)| *horse_id == id)
        .map(|(_, grade, speed, stamina_cost)| (*grade, *speed, *stamina_cost))
}

