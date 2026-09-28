use bitflags::bitflags;

bitflags! {
    pub struct AttributeLocation: u64 {
        const STRUCT = 1 << 0;
        const ENUM = 1 << 1;
        const BITFIELD = 1 << 2;
        const CONSTANT = 1 << 3;
        const FUNCTION = 1 << 4;
        const PIPELINE = 1 << 5;
        const PARAM = 1 << 6;

        const STRUCT_FIELD = 1 << 7;
        const ENUM_VALUE = 1 << 8;
        const BITFIELD_FIELD = 1 << 9;
        const PARAM_FIELD = 1 << 10;

        const FUNCTION_PARAMETER = 1 << 11;
        const FUNCTION_RETURN = 1 << 12;
    }
}

impl AttributeLocation {
    pub const fn types() -> Self {
        return Self::STRUCT.union(Self::ENUM).union(Self::BITFIELD);
    }

    pub const fn members() -> Self {
        return Self::STRUCT_FIELD
            .union(Self::ENUM_VALUE)
            .union(Self::BITFIELD_FIELD)
            .union(Self::PARAM_FIELD);
    }

    pub const fn containers() -> Self {
        return Self::STRUCT
            .union(Self::ENUM)
            .union(Self::BITFIELD)
            .union(Self::PARAM);
    }
}
