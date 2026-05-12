use pulumi_gestalt_model::{
    FromPulumiValue as FromPulumiValueTrait, PulumiValue, PulumiValueContent,
};
use pulumi_gestalt_model_macros::FromPulumiValue;
use rootcause::Result;
use std::collections::HashSet;

#[derive(Debug, PartialEq, FromPulumiValue)]
struct MyStruct {
    a: String,
    b: i32,
    c: Option<String>,
}

#[test]
fn test_derive_struct_success() {
    let value = PulumiValue {
        content: PulumiValueContent::Object(vec![
            (
                "a".to_string(),
                PulumiValue {
                    content: PulumiValueContent::String("hello".to_string()),
                    secret: false,
                    dependencies: HashSet::new(),
                },
            ),
            (
                "b".to_string(),
                PulumiValue {
                    content: PulumiValueContent::Integer(42),
                    secret: false,
                    dependencies: HashSet::new(),
                },
            ),
            (
                "c".to_string(),
                PulumiValue {
                    content: PulumiValueContent::None,
                    secret: false,
                    dependencies: HashSet::new(),
                },
            ),
            (
                "extra".to_string(),
                PulumiValue {
                    content: PulumiValueContent::String("ignored".to_string()),
                    secret: false,
                    dependencies: HashSet::new(),
                },
            ),
        ]),
        secret: false,
        dependencies: HashSet::new(),
    };

    let result = MyStruct::from_pulumi_value(&value).unwrap();
    assert_eq!(
        result,
        MyStruct {
            a: "hello".to_string(),
            b: 42,
            c: None,
        }
    );
}

#[test]
fn test_derive_struct_non_object_error() {
    let value = PulumiValue {
        content: PulumiValueContent::Integer(1),
        secret: false,
        dependencies: HashSet::new(),
    };

    let result: Result<MyStruct> = MyStruct::from_pulumi_value(&value);
    assert!(result.is_err());
    assert!(
        result
            .err()
            .unwrap()
            .to_string()
            .contains("Expected Object")
    );
}

#[test]
fn test_derive_struct_missing_field_error() {
    let value = PulumiValue {
        content: PulumiValueContent::Object(vec![
            (
                "a".to_string(),
                PulumiValue {
                    content: PulumiValueContent::String("hello".to_string()),
                    secret: false,
                    dependencies: HashSet::new(),
                },
            ),
            (
                "c".to_string(),
                PulumiValue {
                    content: PulumiValueContent::None,
                    secret: false,
                    dependencies: HashSet::new(),
                },
            ),
        ]),
        secret: false,
        dependencies: HashSet::new(),
    };

    let result: Result<MyStruct> = MyStruct::from_pulumi_value(&value);
    assert!(result.is_err());
    assert!(
        result
            .err()
            .unwrap()
            .to_string()
            .contains("Missing field 'b'")
    );
}

#[test]
fn test_derive_struct_field_conversion_context() {
    let value = PulumiValue {
        content: PulumiValueContent::Object(vec![
            (
                "a".to_string(),
                PulumiValue {
                    content: PulumiValueContent::String("hello".to_string()),
                    secret: false,
                    dependencies: HashSet::new(),
                },
            ),
            (
                "b".to_string(),
                PulumiValue {
                    content: PulumiValueContent::String("not-int".to_string()),
                    secret: false,
                    dependencies: HashSet::new(),
                },
            ),
            (
                "c".to_string(),
                PulumiValue {
                    content: PulumiValueContent::None,
                    secret: false,
                    dependencies: HashSet::new(),
                },
            ),
        ]),
        secret: false,
        dependencies: HashSet::new(),
    };

    let result: Result<MyStruct> = MyStruct::from_pulumi_value(&value);
    assert!(result.is_err());
    assert!(
        result
            .err()
            .unwrap()
            .to_string()
            .contains("Failed to convert field 'b' to i32")
    );
}
