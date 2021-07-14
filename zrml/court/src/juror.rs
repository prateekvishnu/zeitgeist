use crate::JurorStatus;

/// * Types
///
/// * `B`: Balance
#[derive(parity_scale_codec::Decode, parity_scale_codec::Encode)]
pub struct Juror<B> {
    pub(crate) staked: B,
    pub(crate) status: JurorStatus,
}