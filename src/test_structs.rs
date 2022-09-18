#[public]
struct OnePrivateField {
    value: i32,
}

#[public]
struct OnePublicField {
    pub value: i32,
}

#[public]
pub struct AlreadyPublicStructWithOneField {
    pub value: i32,
}

#[public]
struct TwoFieldsOnePublicOnePrivate {
    pub value: i32,
    second_value: String,
}

#[public]
struct NestedStruct {
    nested_val: bool,
}

#[public]
struct StructWithNestedStructAndOtherField {
    value: i32,
    nested: NestedStruct,
}
