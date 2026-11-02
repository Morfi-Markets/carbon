//! Discriminators of the current Markets CPI events.

/// Anchor `emit_cpi!` self-CPI tag. Every event payload starts with it, followed
/// by the event discriminator.
pub const ANCHOR_EVENT_TAG: [u8; 8] = [0xe4, 0x45, 0xa5, 0x2e, 0x51, 0xcb, 0x9a, 0x1d];

/// ABI name and discriminator of a current CPI event.
///
/// The discriminator is the 8 bytes after [`ANCHOR_EVENT_TAG`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventLayout {
    pub name: &'static str,
    pub discriminator: [u8; 8],
}

impl EventLayout {
    /// Finds a current event layout by its ABI name.
    pub fn named(name: &str) -> Option<&'static Self> {
        EVENT_LAYOUTS.iter().find(|layout| layout.name == name)
    }

    /// Finds the current event layout of a tagged event payload.
    pub fn for_data(data: &[u8]) -> Option<&'static Self> {
        if data.get(..8)? != ANCHOR_EVENT_TAG {
            return None;
        }
        let discriminator = data.get(8..16)?;
        EVENT_LAYOUTS
            .iter()
            .find(|layout| layout.discriminator == discriminator)
    }
}

/// Every current Markets CPI event, one for each `CurrentEvent` variant.
pub const EVENT_LAYOUTS: &[EventLayout] = &[
    EventLayout {
        name: "CidOutcomeAddedEvent",
        discriminator: [186, 132, 249, 50, 0, 113, 73, 219],
    },
    EventLayout {
        name: "ExpiredWinningTokensSweptEvent",
        discriminator: [246, 92, 151, 177, 151, 119, 89, 235],
    },
    EventLayout {
        name: "LogicalResolutionEvent",
        discriminator: [49, 225, 126, 205, 231, 49, 228, 253],
    },
    EventLayout {
        name: "MarketBoostedEvent",
        discriminator: [148, 58, 106, 80, 170, 10, 214, 75],
    },
    EventLayout {
        name: "MarketFundingCompleted",
        discriminator: [186, 180, 149, 230, 193, 127, 207, 14],
    },
    EventLayout {
        name: "MarketInitializedEvent",
        discriminator: [70, 173, 96, 202, 100, 143, 45, 25],
    },
    EventLayout {
        name: "MarketQuoteCustodyClosedEvent",
        discriminator: [66, 205, 127, 209, 198, 254, 11, 58],
    },
    EventLayout {
        name: "PlatformRewardPayoutWalletUpdated",
        discriminator: [255, 78, 216, 17, 81, 169, 241, 77],
    },
    EventLayout {
        name: "PoolRewardClaimed",
        discriminator: [193, 230, 14, 12, 222, 59, 177, 196],
    },
    EventLayout {
        name: "PoolRewardsCollected",
        discriminator: [170, 136, 96, 136, 162, 173, 178, 219],
    },
    EventLayout {
        name: "PositionSlotLifecycleEvent",
        discriminator: [59, 115, 66, 113, 13, 18, 92, 238],
    },
    EventLayout {
        name: "PrincipalCashLiabilitiesSweptEvent",
        discriminator: [59, 240, 33, 183, 169, 232, 169, 85],
    },
    EventLayout {
        name: "PrincipalCashResolutionEvent",
        discriminator: [25, 79, 94, 149, 86, 124, 75, 169],
    },
    EventLayout {
        name: "PrincipalOpinionCoinLaunchEvent",
        discriminator: [185, 149, 242, 59, 8, 79, 33, 26],
    },
    EventLayout {
        name: "PrincipalTradeEvent",
        discriminator: [83, 144, 146, 208, 188, 4, 54, 100],
    },
    EventLayout {
        name: "RewardPayoutWalletUpdated",
        discriminator: [198, 17, 227, 92, 73, 2, 193, 124],
    },
    EventLayout {
        name: "UserPositionsClosedEvent",
        discriminator: [96, 134, 135, 165, 241, 44, 195, 206],
    },
    EventLayout {
        name: "UserPositionsInitializedEvent",
        discriminator: [217, 200, 216, 124, 67, 56, 30, 254],
    },
    EventLayout {
        name: "UserPositionsResizedEvent",
        discriminator: [64, 42, 117, 65, 221, 11, 195, 212],
    },
    EventLayout {
        name: "WalletBackerEntitlementForfeitedEvent",
        discriminator: [42, 21, 93, 240, 227, 27, 187, 239],
    },
    EventLayout {
        name: "WalletBackerFeeClaimedEvent",
        discriminator: [202, 26, 10, 175, 78, 7, 139, 202],
    },
    EventLayout {
        name: "WalletBackingEvent",
        discriminator: [17, 194, 171, 96, 104, 215, 183, 231],
    },
    EventLayout {
        name: "WalletCashEntitlementClearedEvent",
        discriminator: [91, 41, 7, 225, 44, 210, 235, 241],
    },
    EventLayout {
        name: "WalletExpiredBackingRefundEvent",
        discriminator: [31, 67, 144, 74, 210, 254, 192, 116],
    },
    EventLayout {
        name: "WalletOutcomeEntitlementClearedEvent",
        discriminator: [124, 34, 190, 92, 76, 98, 55, 165],
    },
    EventLayout {
        name: "WalletOutcomeTokensWithdrawnEvent",
        discriminator: [89, 172, 0, 172, 78, 17, 91, 221],
    },
    EventLayout {
        name: "WalletPrincipalPrizeClaimedEvent",
        discriminator: [142, 210, 10, 124, 97, 143, 162, 184],
    },
    EventLayout {
        name: "WalletPrizeClaimedEvent",
        discriminator: [91, 242, 254, 29, 197, 30, 243, 161],
    },
    EventLayout {
        name: "WalletPrizeEntitlementClearedEvent",
        discriminator: [100, 172, 112, 83, 93, 232, 214, 201],
    },
    EventLayout {
        name: "WalletTradeEvent",
        discriminator: [251, 216, 11, 8, 56, 243, 152, 247],
    },
    EventLayout {
        name: "WinnerTokenLaunchEvent",
        discriminator: [3, 191, 160, 254, 234, 19, 68, 132],
    },
    EventLayout {
        name: "WinningOutcomeVaultClosedEvent",
        discriminator: [138, 219, 104, 48, 39, 132, 185, 93],
    },
];
