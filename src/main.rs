mod test_structs;

#[macro_use]
extern crate make_fields_public;

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_structs::*;

    #[test]
    fn should_make_private_field_of_struct_public() {
        let res = OnePrivateField {
            value: 1,
        };

        assert_eq!(res.value, 1);
    }

    #[test]
    fn should_keep_public_field_of_struct_public() {
        let res = OnePublicField {
            value: 1,
        };

        assert_eq!(res.value, 1);
    }

    #[test]
    fn should_keep_struct_public() {
        let res = AlreadyPublicStructWithOneField {
            value: 1,
        };

        assert_eq!(res.value, 1);
    }

    #[test]
    fn should_set_multiple_fields_to_public() {
        let res = TwoFieldsOnePublicOnePrivate {
            value: 1,
            second_value: "a string".to_string()
        };

        assert_eq!(res.value, 1);
        assert_eq!(res.second_value, "a string".to_string());
    }

    #[test]
    fn should_also_set_nested_fields_to_public() {
        let res = StructWithNestedStructAndOtherField {
            value: 1,
            nested: NestedStruct {
                nested_val: true,
            },
        };

        assert_eq!(res.value, 1);
        assert_eq!(res.nested.nested_val, true);
    }
}