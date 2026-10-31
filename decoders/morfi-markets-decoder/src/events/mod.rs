//! Carbon shadow structs for the markets program events.

mod backer_fee_claimed_event;
mod backing_event;
mod gift_claimed;
mod gift_created;
mod gift_kind;
mod graduation_event;
mod market_boosted_event;
mod market_created_event;
mod market_funding_completed;
mod prize_claimed_event;
mod resolution_event;
mod sell_cap_applied;
mod trade_event;

pub use backer_fee_claimed_event::BackerFeeClaimedEvent;
pub use backing_event::BackingEvent;
pub use gift_claimed::GiftClaimed;
pub use gift_created::GiftCreated;
pub use gift_kind::GiftKind;
pub use graduation_event::GraduationEvent;
pub use market_boosted_event::MarketBoostedEvent;
pub use market_created_event::MarketCreatedEvent;
pub use market_funding_completed::MarketFundingCompleted;
pub use prize_claimed_event::PrizeClaimedEvent;
pub use resolution_event::ResolutionEvent;
pub use sell_cap_applied::SellCapApplied;
pub use trade_event::TradeEvent;
