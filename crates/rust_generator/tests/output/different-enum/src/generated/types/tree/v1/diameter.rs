#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub enum Diameter {
    sixinch,
    twelveinch,
}

impl pulumi_gestalt_rust::__private::serde::Serialize for Diameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Diameter::sixinch => 6.0,
            Diameter::twelveinch => 12.0,
        };
        serializer.serialize_f64(value)
    }
}

impl<'de> pulumi_gestalt_rust::__private::serde::Deserialize<'de> for Diameter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let f = f64::deserialize(deserializer)?;
        match f {
            6.0 => Ok(Diameter::sixinch),
            12.0 => Ok(Diameter::twelveinch),
            _ => Err(serde::de::Error::custom(format!("Invalid enum value: {}", f))),
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for Diameter {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        let value: f64 = match self {
            Diameter::sixinch => 6.0,
            Diameter::twelveinch => 12.0,
        };
        std::future::ready(pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue {
            content: pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent::Number(value),
            secret: false,
            dependencies: std::collections::HashSet::new(),
        })
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for Diameter {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;

        match &value.content {
            PulumiValueContent::Number(f) => match f {
                6.0 => Ok(Diameter::sixinch),
                12.0 => Ok(Diameter::twelveinch),
                _ => bail!("Invalid number enum value: {}", f),
            },
            _ => bail!("Expected Number, got {:?}", value.content),
        }
    }
}
