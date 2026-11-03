//! Typed current CPI events and discriminator-based parent instruction contexts.

mod events;
pub use events::*;

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
    RewardPayoutWalletUpdated(RewardPayoutWalletUpdated),
    TradeEvent(TradeEvent),
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
    WinnerTokenLaunchEvent(WinnerTokenLaunchEvent),
    WinningOutcomeVaultClosedEvent(WinningOutcomeVaultClosedEvent),
}
mod decode;
mod instruction_context;
pub use crate::layout::{InstructionLayout, INSTRUCTION_LAYOUTS};
pub use instruction_context::CurrentInstruction;
mod instruction_accounts;

pub use instruction_accounts::IncompleteInstructionAccounts;
