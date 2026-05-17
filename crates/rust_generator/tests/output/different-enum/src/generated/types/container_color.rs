#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub enum ContainerColor {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "yellow")]
    Yellow,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ContainerColor {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        let value = match self {
            ContainerColor::Red => "red".to_string(),
            ContainerColor::Blue => "blue".to_string(),
            ContainerColor::Yellow => "yellow".to_string(),
        };
        std::future::ready(pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue {
            content: pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent::String(value),
            secret: false,
            dependencies: std::collections::HashSet::new(),
        })
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ContainerColor {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;

        match &value.content {
            PulumiValueContent::String(s) => match s.as_str() {
                "red" => Ok(ContainerColor::Red),
                "blue" => Ok(ContainerColor::Blue),
                "yellow" => Ok(ContainerColor::Yellow),
                _ => bail!("Invalid string enum value: {}", s),
            },
            _ => bail!("Expected String, got {:?}", value.content),
        }
    }
}
