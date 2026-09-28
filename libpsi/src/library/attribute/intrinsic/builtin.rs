use crate::library::attribute::{
    Attribute, AttributeError, AttributeParameterKind, location::AttributeLocation,
    parameter::AttributeParameter,
};

pub enum BuiltinAttribute {
    Position,
    VertexIndex,
}

impl Attribute for BuiltinAttribute {
    const ALLOWED_LOCATIONS: AttributeLocation = AttributeLocation::STRUCT_FIELD
        .union(AttributeLocation::FUNCTION_PARAMETER)
        .union(AttributeLocation::FUNCTION_RETURN);

    const PATH: &[&str] = &["builtin"];

    fn parse(params: &[AttributeParameter<'_>]) -> Result<Self, AttributeError> {
        let [param] = params else {
            return Err(AttributeError::ArgumentCount {
                expected: 1,
                actual: params.len(),
            });
        };

        let AttributeParameter::IdentifierPath(path) = param else {
            return Err(AttributeError::InvalidArgument {
                index: 0,
                expected: AttributeParameterKind::IdentifierPath,
            });
        };

        return match *path {
            ["position"] => Ok(Self::Position),
            ["vertex_index"] => Ok(Self::VertexIndex),
            _ => Err(AttributeError::InvalidValue {
                index: 0,
                message: "unknown builtin".into(),
            }),
        };
    }
}
