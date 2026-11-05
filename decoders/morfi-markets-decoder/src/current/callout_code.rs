//! Trade events emitted before the Markets callout upgrade end without `callout_code`.
use carbon_core::borsh::{self, io};

/// Borsh `Option<[u8; 8]>`, read as `None` when the payload ends before its tag.
pub(super) fn deserialize<R: io::Read>(reader: &mut R) -> io::Result<Option<[u8; 8]>> {
    let mut tag = [0u8; 1];
    if reader.read(&mut tag)? == 0 {
        return Ok(None);
    }
    match tag[0] {
        0 => Ok(None),
        1 => <[u8; 8] as borsh::BorshDeserialize>::deserialize_reader(reader).map(Some),
        other => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid callout_code option tag {other}"),
        )),
    }
}
