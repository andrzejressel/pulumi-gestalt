use pulumi_gestalt_model::__private::rootcause::{Result, bail};
use pulumi_gestalt_model::{FromPulumiValue, PulumiValue, ToPulumiValue};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum OneOf2<A: Serialize + Debug, B: Serialize + Debug> {
    Left(A),
    Right(B),
}

impl<A: Serialize + Debug, B: Serialize + Debug> OneOf2<A, B> {
    pub fn left(a: A) -> Self {
        OneOf2::Left(a)
    }
    pub fn right(b: B) -> Self {
        OneOf2::Right(b)
    }
}

impl<A, B> ToPulumiValue for OneOf2<A, B>
where
    A: Serialize + Debug + ToPulumiValue,
    B: Serialize + Debug + ToPulumiValue,
{
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> {
        async move {
            match self {
                OneOf2::Left(a) => a.to_pulumi_value().await,
                OneOf2::Right(b) => b.to_pulumi_value().await,
            }
        }
    }
}

impl<A, B> FromPulumiValue for OneOf2<A, B>
where
    A: Serialize + Debug + FromPulumiValue,
    B: Serialize + Debug + FromPulumiValue,
{
    fn from_pulumi_value(value: &PulumiValue) -> Result<Self> {
        match A::from_pulumi_value(value) {
            Ok(v) => Ok(Self::Left(v)),
            Err(err_a) => match B::from_pulumi_value(value) {
                Ok(v) => Ok(Self::Right(v)),
                Err(err_b) => bail!(
                    "Failed to convert PulumiValue to OneOf2<{}, {}>: left variant failed: {}; right variant failed: {}",
                    std::any::type_name::<A>(),
                    std::any::type_name::<B>(),
                    err_a,
                    err_b
                ),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOf3<A: Serialize + Debug, B: Serialize + Debug, C: Serialize + Debug> {
    Left(A),
    Middle(B),
    Right(C),
}

impl<A: Serialize + Debug, B: Serialize + Debug, C: Serialize + Debug> OneOf3<A, B, C> {
    pub fn left(a: A) -> Self {
        OneOf3::Left(a)
    }
    pub fn middle(b: B) -> Self {
        OneOf3::Middle(b)
    }
    pub fn right(c: C) -> Self {
        OneOf3::Right(c)
    }
}

impl<A, B, C> ToPulumiValue for OneOf3<A, B, C>
where
    A: Serialize + Debug + ToPulumiValue,
    B: Serialize + Debug + ToPulumiValue,
    C: Serialize + Debug + ToPulumiValue,
{
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> {
        async move {
            match self {
                OneOf3::Left(a) => a.to_pulumi_value().await,
                OneOf3::Middle(b) => b.to_pulumi_value().await,
                OneOf3::Right(c) => c.to_pulumi_value().await,
            }
        }
    }
}

impl<A, B, C> FromPulumiValue for OneOf3<A, B, C>
where
    A: Serialize + Debug + FromPulumiValue,
    B: Serialize + Debug + FromPulumiValue,
    C: Serialize + Debug + FromPulumiValue,
{
    fn from_pulumi_value(value: &PulumiValue) -> Result<Self> {
        match A::from_pulumi_value(value) {
            Ok(v) => Ok(Self::Left(v)),
            Err(err_a) => match B::from_pulumi_value(value) {
                Ok(v) => Ok(Self::Middle(v)),
                Err(err_b) => match C::from_pulumi_value(value) {
                    Ok(v) => Ok(Self::Right(v)),
                    Err(err_c) => bail!(
                        "Failed to convert PulumiValue to OneOf3<{}, {}, {}>: left variant failed: {}; middle variant failed: {}; right variant failed: {}",
                        std::any::type_name::<A>(),
                        std::any::type_name::<B>(),
                        std::any::type_name::<C>(),
                        err_a,
                        err_b,
                        err_c
                    ),
                },
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OneOf4<
    A: Serialize + Debug,
    B: Serialize + Debug,
    C: Serialize + Debug,
    D: Serialize + Debug,
> {
    Left(A),
    Middle1(B),
    Middle2(C),
    Right(D),
}

impl<A: Serialize + Debug, B: Serialize + Debug, C: Serialize + Debug, D: Serialize + Debug>
    OneOf4<A, B, C, D>
{
    pub fn left(a: A) -> Self {
        OneOf4::Left(a)
    }
    pub fn middle1(b: B) -> Self {
        OneOf4::Middle1(b)
    }
    pub fn middle2(c: C) -> Self {
        OneOf4::Middle2(c)
    }
    pub fn right(d: D) -> Self {
        OneOf4::Right(d)
    }
}

impl<A, B, C, D> ToPulumiValue for OneOf4<A, B, C, D>
where
    A: Serialize + Debug + ToPulumiValue,
    B: Serialize + Debug + ToPulumiValue,
    C: Serialize + Debug + ToPulumiValue,
    D: Serialize + Debug + ToPulumiValue,
{
    fn to_pulumi_value(&self) -> impl Future<Output = PulumiValue> {
        async move {
            match self {
                OneOf4::Left(a) => a.to_pulumi_value().await,
                OneOf4::Middle1(b) => b.to_pulumi_value().await,
                OneOf4::Middle2(c) => c.to_pulumi_value().await,
                OneOf4::Right(d) => d.to_pulumi_value().await,
            }
        }
    }
}

impl<A, B, C, D> FromPulumiValue for OneOf4<A, B, C, D>
where
    A: Serialize + Debug + FromPulumiValue,
    B: Serialize + Debug + FromPulumiValue,
    C: Serialize + Debug + FromPulumiValue,
    D: Serialize + Debug + FromPulumiValue,
{
    fn from_pulumi_value(value: &PulumiValue) -> Result<Self> {
        match A::from_pulumi_value(value) {
            Ok(v) => Ok(Self::Left(v)),
            Err(err_a) => match B::from_pulumi_value(value) {
                Ok(v) => Ok(Self::Middle1(v)),
                Err(err_b) => match C::from_pulumi_value(value) {
                    Ok(v) => Ok(Self::Middle2(v)),
                    Err(err_c) => match D::from_pulumi_value(value) {
                        Ok(v) => Ok(Self::Right(v)),
                        Err(err_d) => bail!(
                            "Failed to convert PulumiValue to OneOf4<{}, {}, {}, {}>: left variant failed: {}; middle1 variant failed: {}; middle2 variant failed: {}; right variant failed: {}",
                            std::any::type_name::<A>(),
                            std::any::type_name::<B>(),
                            std::any::type_name::<C>(),
                            std::any::type_name::<D>(),
                            err_a,
                            err_b,
                            err_c,
                            err_d
                        ),
                    },
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pulumi_gestalt_model::__private::futures::executor::block_on;
    use pulumi_gestalt_model::PulumiValueContent;
    use std::collections::HashSet;

    fn string_value(value: &str) -> PulumiValue {
        PulumiValue {
            content: PulumiValueContent::String(value.to_string()),
            secret: false,
            dependencies: HashSet::new(),
        }
    }

    #[test]
    fn to_pulumi_value_oneof2_uses_active_variant() {
        let value: OneOf2<i32, String> = OneOf2::right("hello".to_string());
        let pulumi = block_on(value.to_pulumi_value());
        assert_eq!(
            pulumi.content,
            PulumiValueContent::String("hello".to_string())
        );
    }

    #[test]
    fn to_pulumi_value_oneof3_uses_active_variant() {
        let value: OneOf3<i32, String, bool> = OneOf3::middle("hello".to_string());
        let pulumi = block_on(value.to_pulumi_value());
        assert_eq!(
            pulumi.content,
            PulumiValueContent::String("hello".to_string())
        );
    }

    #[test]
    fn to_pulumi_value_oneof4_uses_active_variant() {
        let value: OneOf4<i32, bool, String, f64> = OneOf4::middle2("hello".to_string());
        let pulumi = block_on(value.to_pulumi_value());
        assert_eq!(
            pulumi.content,
            PulumiValueContent::String("hello".to_string())
        );
    }

    #[test]
    fn from_pulumi_value_oneof2_success() {
        let parsed: OneOf2<i32, String> =
            OneOf2::from_pulumi_value(&string_value("hello")).unwrap();
        match parsed {
            OneOf2::Right(value) => assert_eq!(value, "hello"),
            _ => panic!("Expected Right variant"),
        }
    }

    #[test]
    fn from_pulumi_value_oneof3_success() {
        let parsed: OneOf3<i32, String, bool> =
            OneOf3::from_pulumi_value(&string_value("hello")).unwrap();
        match parsed {
            OneOf3::Middle(value) => assert_eq!(value, "hello"),
            _ => panic!("Expected Middle variant"),
        }
    }

    #[test]
    fn from_pulumi_value_oneof4_success() {
        let parsed: OneOf4<i32, bool, String, f64> =
            OneOf4::from_pulumi_value(&string_value("hello")).unwrap();
        match parsed {
            OneOf4::Middle2(value) => assert_eq!(value, "hello"),
            _ => panic!("Expected Middle2 variant"),
        }
    }

    #[test]
    fn from_pulumi_value_ambiguous_prefers_left_to_right() {
        let parsed: OneOf2<String, String> =
            OneOf2::from_pulumi_value(&string_value("hello")).unwrap();
        match parsed {
            OneOf2::Left(value) => assert_eq!(value, "hello"),
            _ => panic!("Expected Left variant"),
        }
    }

    #[test]
    fn from_pulumi_value_failure_includes_oneof_context() {
        let value = PulumiValue {
            content: PulumiValueContent::Boolean(true),
            secret: false,
            dependencies: HashSet::new(),
        };

        let result: Result<OneOf3<i32, String, f64>> = OneOf3::from_pulumi_value(&value);
        assert!(result.is_err());
        let message = result.err().unwrap().to_string();
        assert!(message.contains("Failed to convert PulumiValue to OneOf3"));
        assert!(message.contains("left variant failed"));
        assert!(message.contains("middle variant failed"));
        assert!(message.contains("right variant failed"));
    }
}
