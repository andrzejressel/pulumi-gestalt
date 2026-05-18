#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub enum TreeSize {
    #[serde(rename = "small")]
    Small,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "large")]
    Large,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TreeSize {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        let value = match self {
            TreeSize::Small => "small".to_string(),
            TreeSize::Medium => "medium".to_string(),
            TreeSize::Large => "large".to_string(),
        };
        std::future::ready(pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue {
            content: pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent::String(value),
            secret: false,
            dependencies: std::collections::HashSet::new(),
        })
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TreeSize {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;

        match &value.content {
            PulumiValueContent::String(s) => match s.as_str() {
                "small" => Ok(TreeSize::Small),
                "medium" => Ok(TreeSize::Medium),
                "large" => Ok(TreeSize::Large),
                _ => bail!("Invalid string enum value: {}", s),
            },
            _ => bail!("Expected String, got {:?}", value.content),
        }
    }
}
