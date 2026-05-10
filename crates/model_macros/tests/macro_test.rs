use futures::executor::block_on;
use pulumi_gestalt_model::{Output, PulumiValueContent, ToPulumiValue as ToPulumiValueTrait};
use pulumi_gestalt_model_macros::ToPulumiValue;

#[test]
fn test_derive_struct() {
    #[derive(ToPulumiValue)]
    struct MyStruct {
        a: String,
        b: i64,
        c: Output<bool>,
    }

    let val = MyStruct {
        a: "hello".to_string(),
        b: 42,
        c: Output::new_secret(true),
    };

    let pv = block_on(val.to_pulumi_value());

    if let PulumiValueContent::Object(obj) = pv.content {
        assert_eq!(obj.len(), 3);
        assert_eq!(obj[0].0, "a");
        assert_eq!(
            obj[0].1.content,
            PulumiValueContent::String("hello".to_string())
        );
        assert_eq!(obj[1].0, "b");
        assert_eq!(obj[1].1.content, PulumiValueContent::Integer(42));
        assert_eq!(obj[2].0, "c");
        assert_eq!(obj[2].1.content, PulumiValueContent::Boolean(true));
        assert!(pv.secret); // Because c is secret
    } else {
        panic!("Expected Object");
    }
}
