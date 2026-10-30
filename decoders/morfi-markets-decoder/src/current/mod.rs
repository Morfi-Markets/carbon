//! Exact existing mintless CPI layouts and successful parent instruction contexts.

mod events_0;
pub use events_0::*;
mod events_1;
pub use events_1::*;
mod events_2;
pub use events_2::*;
mod events_3;
pub use events_3::*;

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
mod instruction;
mod serialization;
pub use instruction::CurrentInstruction;
mod instruction_accounts;

pub use instruction_accounts::IncompleteInstructionAccounts;
