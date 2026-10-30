/// Returns the first variant whose discriminator and layout deserialize,
/// matching Carbon 0.12's `try_decode_instructions!`. Carbon 2's macro only
/// builds struct variants with `program_id`/`data`/`accounts` fields.
macro_rules! decode_first_variant {
    ($data:expr, $($variant:path => $ty:ty),* $(,)?) => {{
        use carbon_core::deserialize::CarbonDeserialize;
        $(
            if let Some(decoded) = <$ty>::deserialize($data) {
                Some($variant(decoded))
            } else
        )*
        {
            None
        }
    }};
}

/// Implements `CarbonDeserialize` with the contract of Carbon 0.12's removed
/// derive: the exact discriminator prefix, then Borsh, ignoring trailing bytes.
macro_rules! impl_carbon_deserialize {
    ($ty:ty, $discriminator:literal) => {
        impl carbon_core::deserialize::CarbonDeserialize for $ty {
            const DISCRIMINATOR: &'static [u8] = &$crate::deserialize::discriminator::<
                { ($discriminator.len() - 2) / 2 },
            >($discriminator);

            fn deserialize(data: &[u8]) -> Option<Self> {
                let mut rest = data.strip_prefix(Self::DISCRIMINATOR)?;
                borsh::BorshDeserialize::deserialize(&mut rest).ok()
            }
        }
    };
}

/// Parses a `0x`-prefixed hex discriminator at compile time.
pub(crate) const fn discriminator<const N: usize>(hex: &str) -> [u8; N] {
    let hex = hex.as_bytes();
    assert!(hex.len() == 2 + 2 * N && hex[0] == b'0' && hex[1] == b'x');
    let mut bytes = [0; N];
    let mut index = 0;
    while index < N {
        bytes[index] = (nibble(hex[2 + 2 * index]) << 4) | nibble(hex[3 + 2 * index]);
        index += 1;
    }
    bytes
}

const fn nibble(digit: u8) -> u8 {
    match digit {
        b'0'..=b'9' => digit - b'0',
        b'a'..=b'f' => digit - b'a' + 10,
        b'A'..=b'F' => digit - b'A' + 10,
        _ => panic!("discriminator must be hex"),
    }
}
