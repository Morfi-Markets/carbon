//! Current instruction context recognition without argument decoding.

use crate::layout::InstructionLayout;

/// Raw bytes and ABI account names for a recognized current instruction discriminator.
///
/// Argument validity and account presence are not checked by [`Self::decode`].
/// Use [`Self::accounts`] to map the required positional prefix separately.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub struct CurrentInstruction {
    pub name: String,
    pub data: Vec<u8>,
    pub account_names: Vec<String>,
}
impl CurrentInstruction {
    pub fn decode(data: &[u8]) -> Option<Self> {
        let layout = InstructionLayout::for_data(data)?;
        Some(Self {
            name: layout.name.to_string(),
            data: data.to_vec(),
            account_names: layout
                .account_names
                .iter()
                .map(|name| name.to_string())
                .collect(),
        })
    }
}
