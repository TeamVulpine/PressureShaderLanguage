use thiserror::Error;
use ttmap::TypeMap;

use crate::library::attribute::{location::AttributeLocation, parameter::AttributeParameter};

pub mod intrinsic;
pub mod location;
pub mod parameter;

#[derive(Debug, Error)]
#[error("Attribute '{}' was already registered.", .0.join("::"))]
pub struct DuplicateAttributeError(pub(super) &'static [&'static str]);

pub trait Attribute: Sized {
    const ALLOWED_LOCATIONS: AttributeLocation = AttributeLocation::all();
    const PATH: &[&str];

    fn parse(params: &[AttributeParameter]) -> Result<Self, AttributeError>;
}

#[derive(Debug, Error)]
pub enum AttributeError {
    #[error("expected {expected} arguments, found {actual}")]
    ArgumentCount { expected: usize, actual: usize },

    #[error("invalid argument {index}: expected {expected:?}")]
    InvalidArgument {
        index: usize,
        expected: AttributeParameterKind,
    },

    #[error("invalid value for argument {index}: {message}")]
    InvalidValue { index: usize, message: String },

    #[error("{0}")]
    Custom(String),
}

#[derive(Debug, Clone, Copy)]
pub enum AttributeParameterKind {
    String,
    IdentifierPath,
    Number,
}

pub(super) struct AttributeVtable {
    pub allowed_locations: AttributeLocation,
    pub path: &'static [&'static str],
    pub parse: for<'a> fn(&'a [AttributeParameter<'a>], &mut TypeMap) -> Result<(), AttributeError>,
}

impl AttributeVtable {
    pub const fn of<A: Attribute + ttmap::Type>() -> Self {
        return Self {
            allowed_locations: A::ALLOWED_LOCATIONS,
            path: A::PATH,
            parse: |params, map| {
                let value = A::parse(params)?;

                map.insert(value);

                return Ok(());
            },
        };
    }
}
