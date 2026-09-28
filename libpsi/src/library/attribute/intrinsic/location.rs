use crate::library::attribute::{
    Attribute, AttributeError, AttributeParameterKind, location::AttributeLocation,
    parameter::AttributeParameter,
};

pub struct LocationAttribute(u64);

impl Attribute for LocationAttribute {
    const ALLOWED_LOCATIONS: AttributeLocation = AttributeLocation::STRUCT_FIELD
        .union(AttributeLocation::FUNCTION_PARAMETER)
        .union(AttributeLocation::FUNCTION_RETURN);

    const PATH: &[&str] = &["location"];

    fn parse(params: &[AttributeParameter<'_>]) -> Result<Self, AttributeError> {
        let [param] = params else {
            return Err(AttributeError::ArgumentCount {
                expected: 1,
                actual: params.len(),
            });
        };

        let AttributeParameter::Number(location) = param else {
            return Err(AttributeError::InvalidArgument {
                index: 0,
                expected: AttributeParameterKind::Number,
            });
        };

        return Ok(Self(*location));
    }
}

impl LocationAttribute {
    pub const fn location(&self) -> u64 {
        return self.0;
    }
}
