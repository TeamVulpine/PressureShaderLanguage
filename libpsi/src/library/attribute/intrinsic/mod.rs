use std::collections::HashMap;

use crate::library::attribute::{
    Attribute, AttributeVtable,
    intrinsic::{builtin::BuiltinAttribute, location::LocationAttribute},
};

pub mod builtin;
pub mod location;

fn add_intrinsic_attribute<T: Attribute + ttmap::Type>(
    map: &mut HashMap<&'static [&'static str], AttributeVtable>,
) {
    map.insert(T::PATH, AttributeVtable::of::<T>());
}

pub(in crate::library) fn create_intrinsic_attributes()
-> HashMap<&'static [&'static str], AttributeVtable> {
    let mut map = HashMap::new();

    add_intrinsic_attribute::<BuiltinAttribute>(&mut map);
    add_intrinsic_attribute::<LocationAttribute>(&mut map);

    return map;
}
