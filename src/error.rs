pub use derive_getters::Getters;
use orion_error::OrionError;
pub use orion_error::StructError;
use orion_error::UnifiedReason;
pub use orion_error::conversion::ErrorWith;
pub use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, PartialEq, OrionError)]
pub enum ConfIOReason {
    #[orion_error(identity = "conf.other")]
    Other(String),

    #[orion_error(identity = "conf.no_format_enabled")]
    NoFormatEnabled,

    #[orion_error(transparent)]
    General(UnifiedReason),
}

// Keep legacy alias for compatibility
pub type SerdeReason = ConfIOReason;

impl From<String> for ConfIOReason {
    fn from(s: String) -> Self {
        ConfIOReason::Other(s)
    }
}

impl From<UnifiedReason> for ConfIOReason {
    fn from(r: UnifiedReason) -> Self {
        ConfIOReason::General(r)
    }
}

pub type OrionConfResult<T> = Result<T, StructError<ConfIOReason>>;
pub type OrionConfError = StructError<ConfIOReason>;
