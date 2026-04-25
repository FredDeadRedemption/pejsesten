const PROD_STARTING_HP: i32 = 30;
const PROD_STARTING_MANA: i32 = 1;
const PROD_STARTING_HAND_SIZE: usize = 3;
const PROD_BOT_DELAY_MS: u64 = 750;
const PROB_MULLIGAN: bool = true;


pub const STARTING_HP: i32 = if cfg!(debug_assertions) { 100 } else { PROD_STARTING_HP };
pub const STARTING_MANA: i32 = if cfg!(debug_assertions) { 10 } else { PROD_STARTING_MANA };
pub const MAX_MANA: i32 = 10;
pub const STARTING_EMBERS: i32 = 1;
pub const MAX_EMBERS: i32 = 5;
pub const STARTING_HAND_SIZE: usize = if cfg!(debug_assertions) { 7 } else { PROD_STARTING_HAND_SIZE };
pub const BOT_DELAY_MS: u64 = if cfg!(debug_assertions) { 250 } else { PROD_BOT_DELAY_MS };
// pub const OPEN_CARDS: bool = true;
pub const MULLIGAN: bool = if cfg!(debug_assertions) { true } else { PROB_MULLIGAN };
