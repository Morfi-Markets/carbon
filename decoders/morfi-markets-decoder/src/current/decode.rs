use super::*;
impl CurrentEvent {
    /// Whether `data` starts with the tag and discriminator of a current event,
    /// including a recognized event whose payload is truncated.
    pub fn recognizes(data: &[u8]) -> bool {
        crate::layout::EventLayout::for_data(data).is_some()
    }
    pub fn decode(data: &[u8]) -> Option<Self> {
        market_lifecycle_events::decode_market_lifecycle_events(data)
            .or_else(|| resolution_events::decode_resolution_events(data))
            .or_else(|| trade_and_backing_events::decode_trade_and_backing_events(data))
            .or_else(|| claim_events::decode_claim_events(data))
            .or_else(|| reward_events::decode_reward_events(data))
            .or_else(|| position_events::decode_position_events(data))
            .or_else(|| token_launch_events::decode_token_launch_events(data))
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
            | Self::WalletPrincipalPrizeClaimedEvent(WalletPrincipalPrizeClaimedEvent {
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
