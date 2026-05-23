#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use pulumi_gestalt_providers_typesystem::deep::nested::module::some_resource::SomeResourceArgs;
    use pulumi_gestalt_providers_typesystem::types::{
        IntegerEnum, MyEnum, NumberEnum, UnionCase1, UnionCase2, UnionCaseWithConst1,
        UnionCaseWithConst2,
    };
    use pulumi_gestalt_providers_typesystem::typesystem_server::TypesystemServerArgs;
    use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
    use pulumi_gestalt_rust::{FromPulumiValue, OneOf2, ToPulumiValue};

    #[test]
    fn test_struct_to_from_pulumi_value() {
        let case1 = UnionCase1::builder()
            .field_1("value1".to_string())
            .build_struct();
        let case2 = UnionCase2::builder()
            .field_2("value2".to_string())
            .build_struct();

        let value1 = block_on(case1.to_pulumi_value());
        let value2 = block_on(case2.to_pulumi_value());
        let deserialized_case1: UnionCase1 = UnionCase1::from_pulumi_value(&value1).unwrap();
        let deserialized_case2: UnionCase2 = UnionCase2::from_pulumi_value(&value2).unwrap();
        assert_eq!(deserialized_case1, case1);
        assert_eq!(deserialized_case2, case2);
    }

    #[test]
    fn test_string_enum_to_from_pulumi_value() {
        let enum1 = MyEnum::Value1;
        let enum2 = MyEnum::Value2;
        let enum3 = MyEnum::SpecialCharacters;

        let value1 = block_on(enum1.to_pulumi_value());
        let value2 = block_on(enum2.to_pulumi_value());
        let value3 = block_on(enum3.to_pulumi_value());
        assert!(matches!(value1.content, PulumiValueContent::String(ref s) if s == "VALUE1"));
        assert!(matches!(value2.content, PulumiValueContent::String(ref s) if s == "Value2"));
        assert!(
            matches!(value3.content, PulumiValueContent::String(ref s) if s == "Plants'R'Us")
        );

        let deserialized_enum1: MyEnum = MyEnum::from_pulumi_value(&value1).unwrap();
        let deserialized_enum2: MyEnum = MyEnum::from_pulumi_value(&value2).unwrap();
        let deserialized_enum3: MyEnum = MyEnum::from_pulumi_value(&value3).unwrap();
        assert_eq!(deserialized_enum1, enum1);
        assert_eq!(deserialized_enum2, enum2);
        assert_eq!(deserialized_enum3, enum3);
    }

    #[test]
    fn test_integer_enum_to_from_pulumi_value() {
        let enum1 = IntegerEnum::Value1;
        let enum2 = IntegerEnum::Value2;

        let value1 = block_on(enum1.to_pulumi_value());
        let value2 = block_on(enum2.to_pulumi_value());
        assert!(matches!(value1.content, PulumiValueContent::Integer(1)));
        assert!(matches!(value2.content, PulumiValueContent::Integer(2)));

        let deserialized_enum1: IntegerEnum = IntegerEnum::from_pulumi_value(&value1).unwrap();
        let deserialized_enum2: IntegerEnum = IntegerEnum::from_pulumi_value(&value2).unwrap();
        assert_eq!(deserialized_enum1, enum1);
        assert_eq!(deserialized_enum2, enum2);
    }

    #[test]
    fn test_number_enum_to_from_pulumi_value() {
        let enum1 = NumberEnum::Value1;
        let enum2 = NumberEnum::Value2;

        let value1 = block_on(enum1.to_pulumi_value());
        let value2 = block_on(enum2.to_pulumi_value());
        assert!(matches!(value1.content, PulumiValueContent::Number(n) if n == 1.0));
        assert!(matches!(value2.content, PulumiValueContent::Number(n) if n == 2.0));

        let deserialized_enum1: NumberEnum = NumberEnum::from_pulumi_value(&value1).unwrap();
        let deserialized_enum2: NumberEnum = NumberEnum::from_pulumi_value(&value2).unwrap();
        assert_eq!(deserialized_enum1, enum1);
        assert_eq!(deserialized_enum2, enum2);
    }

    #[test]
    fn test_oneof2_to_from_pulumi_value() {
        let oneof1: OneOf2<UnionCaseWithConst1, UnionCaseWithConst2> = OneOf2::Left(
            UnionCaseWithConst1::builder()
                .field_1("value1".to_string())
                .build_struct(),
        );
        let oneof2: OneOf2<UnionCaseWithConst1, UnionCaseWithConst2> = OneOf2::Right(
            UnionCaseWithConst2::builder()
                .field_2("value2".to_string())
                .build_struct(),
        );

        let value1 = block_on(oneof1.to_pulumi_value());
        let value2 = block_on(oneof2.to_pulumi_value());

        let deserialized1: OneOf2<UnionCaseWithConst1, UnionCaseWithConst2> =
            OneOf2::from_pulumi_value(&value1).unwrap();
        let deserialized2: OneOf2<UnionCaseWithConst1, UnionCaseWithConst2> =
            OneOf2::from_pulumi_value(&value2).unwrap();
        assert_eq!(deserialized1, oneof1);
        assert_eq!(deserialized2, oneof2);
    }

    #[test]
    fn test_to_from_pulumi_value_struct_roundtrip() {
        let case1 = UnionCase1::builder()
            .field_1("value1".to_string())
            .build_struct();
        let pulumi_value = block_on(case1.to_pulumi_value());
        let deserialized = UnionCase1::from_pulumi_value(&pulumi_value).unwrap();
        assert_eq!(deserialized, case1);
    }

    #[test]
    fn test_to_from_pulumi_value_enum_roundtrip() {
        let enum_value = MyEnum::SpecialCharacters;
        let pulumi_value = block_on(enum_value.to_pulumi_value());
        let deserialized = MyEnum::from_pulumi_value(&pulumi_value).unwrap();
        assert_eq!(deserialized, enum_value);
    }

    #[allow(dead_code)]
    fn compilation_test() {
        let ctx = get_context();
        // String
        let output = ctx.new_output(&"Hello, World!".to_string());

        let _ = TypesystemServerArgs::builder().required_string_input("&str");
        let _ = TypesystemServerArgs::builder().required_string_input("String".to_string());
        let _ = TypesystemServerArgs::builder().required_string_input(output.clone());

        let _ = TypesystemServerArgs::builder().optional_string_input("&str");
        let _ = TypesystemServerArgs::builder().optional_string_input("String".to_string());
        let _ = TypesystemServerArgs::builder().optional_string_input(output.clone());

        // Vec<String>
        let _ = TypesystemServerArgs::builder().required_string_array(vec!["&str"]);
        let _ = TypesystemServerArgs::builder().required_string_array(vec!["String".to_string()]);
        let _ =
            TypesystemServerArgs::builder().required_string_array(output.clone().map(|s| vec![s]));
        // let _ = TypesystemServerArgs::builder().required_string_array(vec![string_output]);

        let _ = TypesystemServerArgs::builder().optional_string_array(vec!["&str"]);
        let _ = TypesystemServerArgs::builder().optional_string_array(vec!["String".to_string()]);
        let _ =
            TypesystemServerArgs::builder().optional_string_array(output.clone().map(|s| vec![s]));
        // let _ = TypesystemServerArgs::builder().optional_string_array(vec![string_output]);

        // Vec<String> with array
        let _ = TypesystemServerArgs::builder().required_string_array(["&str"]);
        let _ = TypesystemServerArgs::builder().required_string_array(["String".to_string()]);
        let _ =
            TypesystemServerArgs::builder().required_string_array(output.clone().map(|s| vec![s]));
        // let _ = TypesystemServerArgs::builder().required_string_array([string_output]);

        let _ = TypesystemServerArgs::builder().optional_string_array(["&str"]);
        let _ = TypesystemServerArgs::builder().optional_string_array(["String".to_string()]);
        let _ =
            TypesystemServerArgs::builder().optional_string_array(output.clone().map(|s| vec![s]));
        // let _ = TypesystemServerArgs::builder().optional_string_array([string_output]);

        // Union
        let case1 = UnionCase1::builder()
            .field_1("value1".to_string())
            .build_struct();
        let case2 = UnionCase2::builder()
            .field_2("value2".to_string())
            .build_struct();
        let enum_case1_output = ctx.new_output(&case1);
        let enum_case2_output = ctx.new_output(&case2);
        let _ = TypesystemServerArgs::builder().required_union(OneOf2::left(case1));
        let _ = TypesystemServerArgs::builder().required_union(OneOf2::right(case2));
        let _ = TypesystemServerArgs::builder().required_union(enum_case1_output.map(OneOf2::left));
        let _ =
            TypesystemServerArgs::builder().required_union(enum_case2_output.map(OneOf2::right));

        let case1 = UnionCase1::builder()
            .field_1("value1".to_string())
            .build_struct();
        let case2 = UnionCase2::builder()
            .field_2("value2".to_string())
            .build_struct();
        let _ = TypesystemServerArgs::builder().optional_union(OneOf2::left(case1));
        let _ = TypesystemServerArgs::builder().optional_union(OneOf2::right(case2));
        let _ = TypesystemServerArgs::builder().optional_union(enum_case1_output.map(OneOf2::left));
        let _ =
            TypesystemServerArgs::builder().optional_union(enum_case2_output.map(OneOf2::right));

        // // Other types
        // let _ = TypesystemServerArgs::builder()
        //     .required_string_input(42);
        // let _ = TypesystemServerArgs::builder()
        //     .required_string_input(true);
    }

    #[allow(dead_code)]
    fn resource_compilation_test() {
        let ctx = get_context();

        pulumi_gestalt_providers_typesystem::deep::nested::module::some_resource::create(
            ctx,
            "test",
            SomeResourceArgs::builder().build_struct(),
        );
    }

    #[allow(dead_code)]
    fn function_compilation_test() {
        let ctx = get_context();
        pulumi_gestalt_providers_typesystem::functions::deep::nested::module::some_function::invoke(
            ctx,
        );
    }

    #[allow(dead_code)]
    fn types_compilation_test() {
        let _ =
            pulumi_gestalt_providers_typesystem::types::deep::nested::module::SomeType::builder()
                .build_struct();
    }

    fn get_context() -> &'static pulumi_gestalt_rust::Context {
        todo!()
    }
}
