#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub enum Region {
    /// Africa (Cape Town)
    AfSouth1,
    /// Asia Pacific (Hong Kong)
    ApEast1,
    /// Asia Pacific (Tokyo)
    ApNortheast1,
    /// Asia Pacific (Seoul)
    ApNortheast2,
    /// Asia Pacific (Osaka)
    ApNortheast3,
    /// Asia Pacific (Mumbai)
    ApSouth1,
    /// Asia Pacific (Singapore)
    ApSoutheast1,
    /// Asia Pacific (Sydney)
    ApSoutheast2,
    /// Canada (Central)
    CaCentral,
    /// China (Beijing)
    CnNorth1,
    /// China (Ningxia)
    CnNorthwest1,
    /// Europe (Frankfurt)
    EuCentral1,
    /// Europe (Stockholm)
    EuNorth1,
    /// Europe (Ireland)
    EuWest1,
    /// Europe (London)
    EuWest2,
    /// Europe (Paris)
    EuWest3,
    /// Europe (Milan)
    EuSouth1,
    /// Middle East (Bahrain)
    MeSouth1,
    /// South America (São Paulo)
    SaEast1,
    /// AWS GovCloud (US-East)
    UsGovEast1,
    /// AWS GovCloud (US-West)
    UsGovWest1,
    /// US East (N. Virginia)
    UsEast1,
    /// US East (Ohio)
    UsEast2,
    /// US West (N. California)
    UsWest1,
    /// US West (Oregon)
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
