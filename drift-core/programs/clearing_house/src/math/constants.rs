// PRECISIONS
pub const AMM_RESERVE_PRECISION: u128 = 10_000_000_000_000; //expo = -13;
pub const MARK_PRICE_PRECISION: u128 = 10_000_000_000; //expo = -10
pub const QUOTE_PRECISION: u128 = 1_000_000; // expo = -6
pub const FUNDING_PAYMENT_PRECISION: u128 = 10_000; // expo = -4
pub const MARGIN_PRECISION: u128 = 10_000; // expo = -4
pub const PEG_PRECISION: u128 = 1_000; //expo = -3

// PRECISION CONVERSIONS
pub const PRICE_TO_PEG_PRECISION_RATIO: u128 = MARK_PRICE_PRECISION / PEG_PRECISION; // expo: 7
pub const PRICE_TO_PEG_QUOTE_PRECISION_RATIO: u128 = MARK_PRICE_PRECISION / QUOTE_PRECISION; // expo: 4
pub const AMM_TO_QUOTE_PRECISION_RATIO: u128 = AMM_RESERVE_PRECISION / QUOTE_PRECISION; // expo: 7
pub const AMM_TO_QUOTE_PRECISION_RATIO_I128: i128 =
    (AMM_RESERVE_PRECISION / QUOTE_PRECISION) as i128; // expo: 7
pub const AMM_TIMES_PEG_TO_QUOTE_PRECISION_RATIO: u128 =
    AMM_RESERVE_PRECISION * PEG_PRECISION / QUOTE_PRECISION; // expo: 10
pub const QUOTE_TO_BASE_AMT_FUNDING_PRECISION: i128 =
    (AMM_RESERVE_PRECISION * MARK_PRICE_PRECISION * FUNDING_PAYMENT_PRECISION / QUOTE_PRECISION)
        as i128; // expo: 21

// FEE REBATES
pub const SHARE_OF_FEES_ALLOCATED_TO_CLEARING_HOUSE_NUMERATOR: u128 = 1;
pub const SHARE_OF_FEES_ALLOCATED_TO_CLEARING_HOUSE_DENOMINATOR: u128 = 2;
pub const UPDATE_K_ALLOWED_PRICE_CHANGE: u128 = MARK_PRICE_PRECISION / 10;

// TIME PERIODS
pub const ONE_HOUR: i128 = 3600;

// FEES
pub const DEFAULT_FEE_NUMERATOR: u128 = 10;
pub const DEFAULT_FEE_DENOMINATOR: u128 = 10000;
pub const DEFAULT_DISCOUNT_TOKEN_FIRST_TIER_MINIMUM_BALANCE: u64 = 1_000_000_000_000; // 1000
pub const DEFAULT_DISCOUNT_TOKEN_FIRST_TIER_DISCOUNT_NUMERATOR: u128 = 20;
pub const DEFAULT_DISCOUNT_TOKEN_FIRST_TIER_DISCOUNT_DENOMINATOR: u128 = 100;
pub const DEFAULT_DISCOUNT_TOKEN_SECOND_TIER_MINIMUM_BALANCE: u64 = 100_000_000_000;
pub const DEFAULT_DISCOUNT_TOKEN_SECOND_TIER_DISCOUNT_NUMERATOR: u128 = 15;
pub const DEFAULT_DISCOUNT_TOKEN_SECOND_TIER_DISCOUNT_DENOMINATOR: u128 = 100;
pub const DEFAULT_DISCOUNT_TOKEN_THIRD_TIER_MINIMUM_BALANCE: u64 = 10_000_000_000;
pub const DEFAULT_DISCOUNT_TOKEN_THIRD_TIER_DISCOUNT_NUMERATOR: u128 = 10;
pub const DEFAULT_DISCOUNT_TOKEN_THIRD_TIER_DISCOUNT_DENOMINATOR: u128 = 100;
pub const DEFAULT_DISCOUNT_TOKEN_FOURTH_TIER_MINIMUM_BALANCE: u64 = 1_000_000_000;
pub const DEFAULT_DISCOUNT_TOKEN_FOURTH_TIER_DISCOUNT_NUMERATOR: u128 = 5;
pub const DEFAULT_DISCOUNT_TOKEN_FOURTH_TIER_DISCOUNT_DENOMINATOR: u128 = 100;
pub const DEFAULT_REFERRER_REWARD_NUMERATOR: u128 = 5;
pub const DEFAULT_REFERRER_REWARD_DENOMINATOR: u128 = 100;
pub const DEFAULT_REFEREE_DISCOUNT_NUMERATOR: u128 = 5;
pub const DEFAULT_REFEREE_DISCOUNT_DENOMINATOR: u128 = 100;