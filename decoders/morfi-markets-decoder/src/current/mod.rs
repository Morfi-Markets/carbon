//! Typed current CPI events and discriminator-based parent instruction contexts.

mod market_lifecycle_events;
pub use market_lifecycle_events::*;
mod resolution_events;
pub use resolution_events::*;
mod trade_and_backing_events;
pub use trade_and_backing_events::*;
mod claim_events;
pub use claim_events::*;
mod reward_events;
pub use reward_events::*;
mod position_events;
pub use position_events::*;
mod token_launch_events;
pub use token_launch_events::*;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Hash,
    strum_macros::IntoStaticStr,
)]
pub enum CurrentEvent {
    CidOutcomeAddedEvent(CidOutcomeAddedEvent),
    ExpiredWinningTokensSweptEvent(ExpiredWinningTokensSweptEvent),
    LogicalResolutionEvent(LogicalResolutionEvent),
    MarketBoostedEvent(MarketBoostedEvent),
    MarketFundingCompleted(MarketFundingCompleted),
    MarketInitializedEvent(MarketInitializedEvent),
    MarketQuoteCustodyClosedEvent(MarketQuoteCustodyClosedEvent),
    PlatformRewardPayoutWalletUpdated(PlatformRewardPayoutWalletUpdated),
    PoolRewardClaimed(PoolRewardClaimed),
    PoolRewardsCollected(PoolRewardsCollected),
    PositionSlotLifecycleEvent(PositionSlotLifecycleEvent),
    PrincipalCashLiabilitiesSweptEvent(PrincipalCashLiabilitiesSweptEvent),
    PrincipalCashResolutionEvent(PrincipalCashResolutionEvent),
    PrincipalOpinionCoinLaunchEvent(PrincipalOpinionCoinLaunchEvent),
    PrincipalTradeEvent(PrincipalTradeEvent),
    RewardPayoutWalletUpdated(RewardPayoutWalletUpdated),
    UserPositionsClosedEvent(UserPositionsClosedEvent),
    UserPositionsInitializedEvent(UserPositionsInitializedEvent),
    UserPositionsResizedEvent(UserPositionsResizedEvent),
    WalletBackerEntitlementForfeitedEvent(WalletBackerEntitlementForfeitedEvent),
    WalletBackerFeeClaimedEvent(WalletBackerFeeClaimedEvent),
    WalletBackingEvent(WalletBackingEvent),
    WalletCashEntitlementClearedEvent(WalletCashEntitlementClearedEvent),
    WalletExpiredBackingRefundEvent(WalletExpiredBackingRefundEvent),
    WalletOutcomeEntitlementClearedEvent(WalletOutcomeEntitlementClearedEvent),
    WalletOutcomeTokensWithdrawnEvent(WalletOutcomeTokensWithdrawnEvent),
    WalletPrincipalPrizeClaimedEvent(WalletPrincipalPrizeClaimedEvent),
    WalletPrizeClaimedEvent(WalletPrizeClaimedEvent),
    WalletPrizeEntitlementClearedEvent(WalletPrizeEntitlementClearedEvent),
    WalletTradeEvent(WalletTradeEvent),
    WinnerTokenLaunchEvent(WinnerTokenLaunchEvent),
    WinningOutcomeVaultClosedEvent(WinningOutcomeVaultClosedEvent),
}
mod decode;
mod instruction_context;
mod u128_as_string;
pub use instruction_context::CurrentInstruction;
mod instruction_accounts;

pub use instruction_accounts::IncompleteInstructionAccounts;
