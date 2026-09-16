const DEV_STARTING_HP: i32 = 100;
const DEV_MAX_HP: i32 = 100;
const DEV_STARTING_MANA: i32 = 10;
const DEV_MAX_MANA: i32 = 10;
const DEV_STARTING_EMBERS: i32 = 5;
const DEV_MAX_EMBERS: i32 = 5;
const DEV_STARTING_HAND_SIZE: usize = 7;
const DEV_MAX_HAND_SIZE: usize = 10;
const DEV_MAX_BOARD_SIZE: usize = 5;
const DEV_MAX_OMENS: usize = 3;
const DEV_MULLIGAN: bool = true;
const DEV_OPEN_CARDS: bool = true;
const DEV_BOT_DELAY_MS: u64 = 250;

/// Pause between frames when the scenario catalogue is played back visually.
pub const SCENARIO_FRAME_MS: u64 = 100;

const PROD_STARTING_HP: i32 = 30;
const PROD_MAX_HP: i32 = 30;
const PROD_STARTING_MANA: i32 = 1;
const PROD_MAX_MANA: i32 = 10;
const PROD_STARTING_EMBERS: i32 = 1;
const PROD_MAX_EMBERS: i32 = 5;
const PROD_STARTING_HAND_SIZE: usize = 3;
const PROD_MAX_HAND_SIZE: usize = 10;
const PROD_MAX_BOARD_SIZE: usize = 5;
const PROD_MAX_OMENS: usize = 3;
const PROD_MULLIGAN: bool = true;
const PROD_OPEN_CARDS: bool = false;
const PROD_BOT_DELAY_MS: u64 = 750;

/// Release always runs the prod tuning; `--features settings-prod` turns it on in a debug build too.
const DEV: bool = cfg!(debug_assertions) && !cfg!(feature = "settings-prod");

pub const PROFILE: &str = if DEV { "dev" } else { "prod" };

pub const STARTING_HP: i32 = if DEV { DEV_STARTING_HP } else { PROD_STARTING_HP };
pub const MAX_HP: i32 = if DEV { DEV_MAX_HP } else { PROD_MAX_HP };
pub const STARTING_MANA: i32 = if DEV { DEV_STARTING_MANA } else { PROD_STARTING_MANA };
pub const MAX_MANA: i32 = if DEV { DEV_MAX_MANA } else { PROD_MAX_MANA };
pub const STARTING_EMBERS: i32 = if DEV { DEV_STARTING_EMBERS } else { PROD_STARTING_EMBERS };
pub const MAX_EMBERS: i32 = if DEV { DEV_MAX_EMBERS } else { PROD_MAX_EMBERS };
pub const STARTING_HAND_SIZE: usize = if DEV { DEV_STARTING_HAND_SIZE } else { PROD_STARTING_HAND_SIZE };
pub const MAX_HAND_SIZE: usize = if DEV { DEV_MAX_HAND_SIZE } else { PROD_MAX_HAND_SIZE };
pub const MAX_BOARD_SIZE: usize = if DEV { DEV_MAX_BOARD_SIZE } else { PROD_MAX_BOARD_SIZE };
pub const MAX_OMENS: usize = if DEV { DEV_MAX_OMENS } else { PROD_MAX_OMENS };
pub const MULLIGAN: bool = if DEV { DEV_MULLIGAN } else { PROD_MULLIGAN };
pub const OPEN_CARDS: bool = if DEV { DEV_OPEN_CARDS } else { PROD_OPEN_CARDS };
pub const BOT_DELAY_MS: u64 = if DEV { DEV_BOT_DELAY_MS } else { PROD_BOT_DELAY_MS };
