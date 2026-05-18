#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub enum Region {
    /// Africa (Cape Town)
    #[serde(rename = "af-south-1")]
    AfSouth1,
    /// Asia Pacific (Hong Kong)
    #[serde(rename = "ap-east-1")]
    ApEast1,
    /// Asia Pacific (Tokyo)
    #[serde(rename = "ap-northeast-1")]
    ApNortheast1,
    /// Asia Pacific (Seoul)
    #[serde(rename = "ap-northeast-2")]
    ApNortheast2,
    /// Asia Pacific (Osaka)
    #[serde(rename = "ap-northeast-3")]
    ApNortheast3,
    /// Asia Pacific (Mumbai)
    #[serde(rename = "ap-south-1")]
    ApSouth1,
    /// Asia Pacific (Singapore)
    #[serde(rename = "ap-southeast-1")]
    ApSoutheast1,
    /// Asia Pacific (Sydney)
    #[serde(rename = "ap-southeast-2")]
    ApSoutheast2,
    /// Canada (Central)
    #[serde(rename = "ca-central-1")]
    CaCentral,
    /// China (Beijing)
    #[serde(rename = "cn-north-1")]
    CnNorth1,
    /// China (Ningxia)
    #[serde(rename = "cn-northwest-1")]
    CnNorthwest1,
    /// Europe (Frankfurt)
    #[serde(rename = "eu-central-1")]
    EuCentral1,
    /// Europe (Stockholm)
    #[serde(rename = "eu-north-1")]
    EuNorth1,
    /// Europe (Ireland)
    #[serde(rename = "eu-west-1")]
    EuWest1,
    /// Europe (London)
    #[serde(rename = "eu-west-2")]
    EuWest2,
    /// Europe (Paris)
    #[serde(rename = "eu-west-3")]
    EuWest3,
    /// Europe (Milan)
    #[serde(rename = "eu-south-1")]
    EuSouth1,
    /// Middle East (Bahrain)
    #[serde(rename = "me-south-1")]
    MeSouth1,
    /// South America (São Paulo)
    #[serde(rename = "sa-east-1")]
    SaEast1,
    /// AWS GovCloud (US-East)
    #[serde(rename = "us-gov-east-1")]
    UsGovEast1,
    /// AWS GovCloud (US-West)
    #[serde(rename = "us-gov-west-1")]
    UsGovWest1,
    /// US East (N. Virginia)
    #[serde(rename = "us-east-1")]
    UsEast1,
    /// US East (Ohio)
    #[serde(rename = "us-east-2")]
    UsEast2,
    /// US West (N. California)
    #[serde(rename = "us-west-1")]
    UsWest1,
    /// US West (Oregon)
    #[serde(rename = "us-west-2")]
    UsWest2,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for Region {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        let value = match self {
            Region::AfSouth1 => "af-south-1".to_string(),
            Region::ApEast1 => "ap-east-1".to_string(),
            Region::ApNortheast1 => "ap-northeast-1".to_string(),
            Region::ApNortheast2 => "ap-northeast-2".to_string(),
            Region::ApNortheast3 => "ap-northeast-3".to_string(),
            Region::ApSouth1 => "ap-south-1".to_string(),
            Region::ApSoutheast1 => "ap-southeast-1".to_string(),
            Region::ApSoutheast2 => "ap-southeast-2".to_string(),
            Region::CaCentral => "ca-central-1".to_string(),
            Region::CnNorth1 => "cn-north-1".to_string(),
            Region::CnNorthwest1 => "cn-northwest-1".to_string(),
            Region::EuCentral1 => "eu-central-1".to_string(),
            Region::EuNorth1 => "eu-north-1".to_string(),
            Region::EuWest1 => "eu-west-1".to_string(),
            Region::EuWest2 => "eu-west-2".to_string(),
            Region::EuWest3 => "eu-west-3".to_string(),
            Region::EuSouth1 => "eu-south-1".to_string(),
            Region::MeSouth1 => "me-south-1".to_string(),
            Region::SaEast1 => "sa-east-1".to_string(),
            Region::UsGovEast1 => "us-gov-east-1".to_string(),
            Region::UsGovWest1 => "us-gov-west-1".to_string(),
            Region::UsEast1 => "us-east-1".to_string(),
            Region::UsEast2 => "us-east-2".to_string(),
            Region::UsWest1 => "us-west-1".to_string(),
            Region::UsWest2 => "us-west-2".to_string(),
        };
        std::future::ready(pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue {
            content: pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent::String(value),
            secret: false,
            dependencies: std::collections::HashSet::new(),
        })
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for Region {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;

        match &value.content {
            PulumiValueContent::String(s) => match s.as_str() {
                "af-south-1" => Ok(Region::AfSouth1),
                "ap-east-1" => Ok(Region::ApEast1),
                "ap-northeast-1" => Ok(Region::ApNortheast1),
                "ap-northeast-2" => Ok(Region::ApNortheast2),
                "ap-northeast-3" => Ok(Region::ApNortheast3),
                "ap-south-1" => Ok(Region::ApSouth1),
                "ap-southeast-1" => Ok(Region::ApSoutheast1),
                "ap-southeast-2" => Ok(Region::ApSoutheast2),
                "ca-central-1" => Ok(Region::CaCentral),
                "cn-north-1" => Ok(Region::CnNorth1),
                "cn-northwest-1" => Ok(Region::CnNorthwest1),
                "eu-central-1" => Ok(Region::EuCentral1),
                "eu-north-1" => Ok(Region::EuNorth1),
                "eu-west-1" => Ok(Region::EuWest1),
                "eu-west-2" => Ok(Region::EuWest2),
                "eu-west-3" => Ok(Region::EuWest3),
                "eu-south-1" => Ok(Region::EuSouth1),
                "me-south-1" => Ok(Region::MeSouth1),
                "sa-east-1" => Ok(Region::SaEast1),
                "us-gov-east-1" => Ok(Region::UsGovEast1),
                "us-gov-west-1" => Ok(Region::UsGovWest1),
                "us-east-1" => Ok(Region::UsEast1),
                "us-east-2" => Ok(Region::UsEast2),
                "us-west-1" => Ok(Region::UsWest1),
                "us-west-2" => Ok(Region::UsWest2),
                _ => bail!("Invalid string enum value: {}", s),
            },
            _ => bail!("Expected String, got {:?}", value.content),
        }
    }
}
