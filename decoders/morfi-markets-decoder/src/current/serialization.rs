//! Preserve the existing JSON representation of wide ABI integers.
pub(super) fn serialize_u128<S: serde::Serializer>(
    value: &u128,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
}
