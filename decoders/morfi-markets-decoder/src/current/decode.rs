use super::*;
impl CurrentEvent {
    /// Whether `data` starts with the tag and discriminator of a current event,
    /// including a recognized event whose payload is truncated.
    pub fn recognizes(data: &[u8]) -> bool {
        crate::layout::EventLayout::for_data(data).is_some()
    }
    pub fn decode(data: &[u8]) -> Option<Self> {
        decode_first_variant!(
            data,
            Self::CidOutcomeAddedEvent => CidOutcomeAddedEvent,
            Self::ExpiredWinningTokensSweptEvent => ExpiredWinningTokensSweptEvent,
            Self::LogicalResolutionEvent => LogicalResolutionEvent,
            Self::MarketBoostedEvent => MarketBoostedEvent,
            Self::MarketFundingCompleted => MarketFundingCompleted,
            Self::MarketInitializedEvent => MarketInitializedEvent,
            Self::MarketQuoteCustodyClosedEvent => MarketQuoteCustodyClosedEvent,
            Self::PlatformRewardPayoutWalletUpdated => PlatformRewardPayoutWalletUpdated,
            Self::PoolRewardClaimed => PoolRewardClaimed,
            Self::PoolRewardsCollected => PoolRewardsCollected,
            Self::PositionSlotLifecycleEvent => PositionSlotLifecycleEvent,
            Self::PrincipalCashLiabilitiesSweptEvent => PrincipalCashLiabilitiesSweptEvent,
            Self::PrincipalCashResolutionEvent => PrincipalCashResolutionEvent,
            Self::PrincipalOpinionCoinLaunchEvent => PrincipalOpinionCoinLaunchEvent,
            Self::PrincipalTradeEvent => PrincipalTradeEvent,
            Self::RewardPayoutWalletUpdated => RewardPayoutWalletUpdated,
            Self::UserPositionsClosedEvent => UserPositionsClosedEvent,
            Self::UserPositionsInitializedEvent => UserPositionsInitializedEvent,
            Self::UserPositionsResizedEvent => UserPositionsResizedEvent,
            Self::WalletBackerEntitlementForfeitedEvent => WalletBackerEntitlementForfeitedEvent,
            Self::WalletBackerFeeClaimedEvent => WalletBackerFeeClaimedEvent,
            Self::WalletBackingEvent => WalletBackingEvent,
            Self::WalletCashEntitlementClearedEvent => WalletCashEntitlementClearedEvent,
            Self::WalletExpiredBackingRefundEvent => WalletExpiredBackingRefundEvent,
            Self::WalletOutcomeEntitlementClearedEvent => WalletOutcomeEntitlementClearedEvent,
            Self::WalletOutcomeTokensWithdrawnEvent => WalletOutcomeTokensWithdrawnEvent,
            Self::WalletPrincipalBonusClaimedEvent => WalletPrincipalBonusClaimedEvent,
            Self::WalletPrizeClaimedEvent => WalletPrizeClaimedEvent,
            Self::WalletPrizeEntitlementClearedEvent => WalletPrizeEntitlementClearedEvent,
            Self::WalletTradeEvent => WalletTradeEvent,
            Self::WinnerTokenLaunchEvent => WinnerTokenLaunchEvent,
            Self::WinningOutcomeVaultClosedEvent => WinningOutcomeVaultClosedEvent,
        )
    }
    pub fn name(&self) -> &'static str {
        self.into()
    }
    pub fn timestamp(&self) -> Option<i64> {
        match self {
            Self::CidOutcomeAddedEvent(CidOutcomeAddedEvent { ts, .. })
            | Self::ExpiredWinningTokensSweptEvent(ExpiredWinningTokensSweptEvent { ts, .. })
            | Self::LogicalResolutionEvent(LogicalResolutionEvent { ts, .. })
            | Self::MarketBoostedEvent(MarketBoostedEvent { ts, .. })
            | Self::MarketInitializedEvent(MarketInitializedEvent { ts, .. })
            | Self::MarketQuoteCustodyClosedEvent(MarketQuoteCustodyClosedEvent { ts, .. })
            | Self::PlatformRewardPayoutWalletUpdated(PlatformRewardPayoutWalletUpdated {
                ts,
                ..
            })
            | Self::PoolRewardClaimed(PoolRewardClaimed { ts, .. })
            | Self::PoolRewardsCollected(PoolRewardsCollected { ts, .. })
            | Self::PositionSlotLifecycleEvent(PositionSlotLifecycleEvent { ts, .. })
            | Self::PrincipalCashLiabilitiesSweptEvent(PrincipalCashLiabilitiesSweptEvent {
                ts,
                ..
            })
            | Self::PrincipalCashResolutionEvent(PrincipalCashResolutionEvent { ts, .. })
            | Self::PrincipalOpinionCoinLaunchEvent(PrincipalOpinionCoinLaunchEvent {
                ts, ..
            })
            | Self::PrincipalTradeEvent(PrincipalTradeEvent { ts, .. })
            | Self::RewardPayoutWalletUpdated(RewardPayoutWalletUpdated { ts, .. })
            | Self::WalletBackerEntitlementForfeitedEvent(
                WalletBackerEntitlementForfeitedEvent { ts, .. },
            )
            | Self::WalletBackerFeeClaimedEvent(WalletBackerFeeClaimedEvent { ts, .. })
            | Self::WalletBackingEvent(WalletBackingEvent { ts, .. })
            | Self::WalletCashEntitlementClearedEvent(WalletCashEntitlementClearedEvent {
                ts,
                ..
            })
            | Self::WalletExpiredBackingRefundEvent(WalletExpiredBackingRefundEvent {
                ts, ..
            })
            | Self::WalletOutcomeEntitlementClearedEvent(WalletOutcomeEntitlementClearedEvent {
                ts,
                ..
            })
            | Self::WalletOutcomeTokensWithdrawnEvent(WalletOutcomeTokensWithdrawnEvent {
                ts,
                ..
            })
            | Self::WalletPrincipalBonusClaimedEvent(WalletPrincipalBonusClaimedEvent {
                ts, ..
            })
            | Self::WalletPrizeClaimedEvent(WalletPrizeClaimedEvent { ts, .. })
            | Self::WalletPrizeEntitlementClearedEvent(WalletPrizeEntitlementClearedEvent {
                ts,
                ..
            })
            | Self::WalletTradeEvent(WalletTradeEvent { ts, .. })
            | Self::WinnerTokenLaunchEvent(WinnerTokenLaunchEvent { ts, .. })
            | Self::WinningOutcomeVaultClosedEvent(WinningOutcomeVaultClosedEvent { ts, .. }) => {
                Some(*ts)
            }
            Self::MarketFundingCompleted(_)
            | Self::UserPositionsClosedEvent(_)
            | Self::UserPositionsInitializedEvent(_)
            | Self::UserPositionsResizedEvent(_) => None,
        }
    }
}
