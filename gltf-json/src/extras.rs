use gltf_derive::Validate as MValidate;
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[cfg(feature = "extras")]
pub use serde_json::value::Value;
#[cfg(feature = "extras")]
use crate::{
    validation::{Error, Validate},
    Path, Root,
};

/// Data type of the `extras` attribute on all glTF objects.
#[cfg(feature = "extras")]
pub type Extras = Option<::std::boxed::Box<Value>>;

#[cfg(feature = "extras")]
impl Validate for Extras {
    fn validate<P, R>(&self, _root: &Root, _path: P, _report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
    }
}

/// Data type of the `extras` attribute on all glTF objects.
#[cfg(not(feature = "extras"))]
pub type Extras = Void;

/// Type representing no user-defined data.
#[derive(Clone, Default, Serialize, Deserialize, MValidate)]
pub struct Void {
    #[serde(default, skip_serializing)]
    _allow_unknown_fields: (),
}

impl fmt::Debug for Void {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{}}")
    }
}

impl fmt::Display for Void {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{}}")
    }
}
