#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub enum SupportedFilterTypes {
    /// Ship to country
    #[serde(rename = "ShipToCountries")]
    ShipToCountries,
    /// Double encryption status
    #[serde(rename = "DoubleEncryptionStatus")]
    DoubleEncryptionStatus,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SupportedFilterTypes {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        let value = match self {
            SupportedFilterTypes::ShipToCountries => "ShipToCountries".to_string(),
            SupportedFilterTypes::DoubleEncryptionStatus => "DoubleEncryptionStatus".to_string(),
        };
        std::future::ready(pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue {
            content: pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent::String(value),
            secret: false,
            dependencies: std::collections::HashSet::new(),
        })
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SupportedFilterTypes {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;

        match &value.content {
            PulumiValueContent::String(s) => match s.as_str() {
                "ShipToCountries" => Ok(SupportedFilterTypes::ShipToCountries),
                "DoubleEncryptionStatus" => Ok(SupportedFilterTypes::DoubleEncryptionStatus),
                _ => bail!("Invalid string enum value: {}", s),
            },
            _ => bail!("Expected String, got {:?}", value.content),
        }
    }
}
