use crate::algebraic_type::AlgebraicType;
use crate::{de::Deserialize, ser::Serialize};

/// A map type from keys of type `key_ty` to values of type `ty`.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[sats(crate = crate)]
pub struct MapType {
    /// The key type of the map.
    pub key_ty: AlgebraicType,
    /// The value type of the map.
    pub ty: AlgebraicType,
}

impl MapType {
    /// Returns a map type with keys of type `key` and values of type `value`.
    pub fn new(key: AlgebraicType, value: AlgebraicType) -> Self {
        Self {
            key_ty: key,
            ty: value,
        }
    }
}
