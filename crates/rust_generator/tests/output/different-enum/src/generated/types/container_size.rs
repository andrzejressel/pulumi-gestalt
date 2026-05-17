#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub enum ContainerSize {
    FourInch,
    SixInch,
    EightInch,
}

impl pulumi_gestalt_rust::__private::serde::Serialize for ContainerSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            ContainerSize::FourInch => 4,
            ContainerSize::SixInch => 6,
            ContainerSize::EightInch => 8,
        };
        serializer.serialize_i64(value)
    }
}

impl<'de> pulumi_gestalt_rust::__private::serde::Deserialize<'de> for ContainerSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let f = i64::deserialize(deserializer)?;
        match f {
            4 => Ok(ContainerSize::FourInch),
            6 => Ok(ContainerSize::SixInch),
            8 => Ok(ContainerSize::EightInch),
            _ => Err(serde::de::Error::custom(format!("Invalid enum value: {}", f))),
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ContainerSize {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        let value: i32 = match self {
            ContainerSize::FourInch => 4,
            ContainerSize::SixInch => 6,
            ContainerSize::EightInch => 8,
        };
        std::future::ready(pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue {
            content: pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent::Integer(value),
            secret: false,
            dependencies: std::collections::HashSet::new(),
        })
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ContainerSize {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;

        match &value.content {
            PulumiValueContent::Integer(i) => match i {
                4 => Ok(ContainerSize::FourInch),
                6 => Ok(ContainerSize::SixInch),
                8 => Ok(ContainerSize::EightInch),
                _ => bail!("Invalid integer enum value: {}", i),
            },
            _ => bail!("Expected Integer, got {:?}", value.content),
        }
    }
}
