//! The `TriState` crate provides a three-valued type equivalent to `Option<bool>`.
//! It also provides several convenience methods for testing tri-state values.

use std::convert::From;

/// A three-valued type equivalent to `Option<bool>`.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum TriState {
    /// The tri-state value signifying definitive truth.
    Yes,

    /// The tri-state value signifying definitive falsity.
    No,

    /// The tri-state value signifying an unknown truth value.
    Unknown
}

impl TriState {
    /// Is `self` equal to `Yes`?
    pub fn yes(self) -> bool { self == TriState::Yes }

    /// Is `self` equal to `No`?
    pub fn no(self) -> bool { self == TriState::No }

    /// Is `self` equal to `Unknown`?
    pub fn unknown(self) -> bool { self == TriState::Unknown }

    /// A mnemonic for `yes()`.
    pub fn definitely(self) -> bool { self.yes() }

    /// A mnemonic for `no()`.
    pub fn definitely_not(self) -> bool { self.no() }

    /// A mnemonic for `unknown()`.
    pub fn maybe(self) -> bool { self.unknown() }

    /// Converts `self` to an `Option<bool>`. Equivalent to `Option::<bool>::from(self)`.
    pub fn to_bool(self) -> Option<bool> {
        match self {
            TriState::Yes => Some(true),
            TriState::No => Some(false),
            TriState::Unknown => None
        }
    }
}

impl From<TriState> for Option<bool> {
    fn from(t: TriState) -> Option<bool> { t.to_bool() }
}

impl From<Option<bool>> for TriState {
    fn from(b: Option<bool>) -> TriState {
        match b {
            Some(true) => TriState::Yes,
            Some(false) => TriState::No,
            None => TriState::Unknown
        }
    }
}

impl From<bool> for TriState {
    fn from(b: bool) -> TriState {
        match b {
            true => TriState::Yes,
            false => TriState::No
        }
    }
}
