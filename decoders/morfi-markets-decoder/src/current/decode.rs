use super::*;
impl CurrentEvent {
    pub fn recognizes(data: &[u8]) -> bool {
        if data.get(..8) != Some(&[0xe4, 0x45, 0xa5, 0x2e, 0x51, 0xcb, 0x9a, 0x1d][..]) {
            return false;
        }
        let Some(bytes) = data.get(8..16) else {
            return false;
        };
        let Ok(bytes) = bytes.try_into() else {
            return false;
        };
        matches!(
            u64::from_le_bytes(bytes),
            15801285013275247802
                | 16958717365243567350
                | 18294802457940648241
                | 5464566924470532756
                | 1067212107484083386
                | 1814263887955012934
                | 4182716808815430978
                | 5616456375964552959
                | 14173175327182743233
                | 15830906553745049770
                | 17175622927828284219
                | 6172720579907547195
                | 12198980726978137881
                | 1882873016001664441
                | 7220964261534797907
                | 8989468845322473926
                | 14898801408647595616
                | 18311134997443561689
                | 15331110601413175872
                | 17274431461513827626
                | 14594767051949742794
                | 16697050986797842961
                | 17432257872863373659
                | 8413004283075642143
                | 11905092220090196604
                | 15950361534853590105
                | 13304353898061550222
                | 11669704891407725147
                | 14544067534169091172
                | 17841277345806932219
                | 9530764611438034691
                | 6753574420069538698
        )
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
