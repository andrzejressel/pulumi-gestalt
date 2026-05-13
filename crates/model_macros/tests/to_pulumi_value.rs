use futures::executor::block_on;
use pulumi_gestalt_model::{Output, PulumiValueContent, ToPulumiValue as ToPulumiValueTrait};
use pulumi_gestalt_model_macros::ToPulumiValue;

#[test]
fn test_derive_struct() {
    #[derive(ToPulumiValue)]
    struct MyStruct {
        a: String,
        b: i32,
        c: Output<bool>,
        d: Option<String>,
    }

    let val = MyStruct {
        a: "hello".to_string(),
        b: 42,
        c: Output::new_secret(true),
        d: None,
    };

    let pv = block_on(val.to_pulumi_value());

    if let PulumiValueContent::Object(obj) = pv.content {
        assert_eq!(obj.len(), 4);
        assert_eq!(obj[0].0, "a");
        assert_eq!(
            obj[0].1.content,
            PulumiValueContent::String("hello".to_string())
        );
        assert_eq!(obj[1].0, "b");
        assert_eq!(obj[1].1.content, PulumiValueContent::Integer(42));
        assert_eq!(obj[2].0, "c");
        assert_eq!(obj[2].1.content, PulumiValueContent::Boolean(true));
        assert_eq!(obj[3].0, "d");
        assert_eq!(obj[3].1.content, PulumiValueContent::None);
        assert!(pv.secret); // Because c is secret
    } else {
        panic!("Expected Object");
    }
}
