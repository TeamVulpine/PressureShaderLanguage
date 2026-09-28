pub enum AttributeParameter<'a> {
    String(&'a str),
    IdentifierPath(&'a [&'a str]),
    Number(u64),
}
