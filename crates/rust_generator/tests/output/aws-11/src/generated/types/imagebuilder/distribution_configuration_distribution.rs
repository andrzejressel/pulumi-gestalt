#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DistributionConfigurationDistribution {
    /// Configuration block with Amazon Machine Image (AMI) distribution settings. Detailed below.
    #[builder(into)]
    #[serde(rename = "amiDistributionConfiguration")]
    pub r#ami_distribution_configuration: Option<Box<super::super::types::imagebuilder::DistributionConfigurationDistributionAmiDistributionConfiguration>>,
    /// Configuration block with container distribution settings. Detailed below.
    #[builder(into)]
    #[serde(rename = "containerDistributionConfiguration")]
    pub r#container_distribution_configuration: Option<Box<super::super::types::imagebuilder::DistributionConfigurationDistributionContainerDistributionConfiguration>>,
    /// Set of Windows faster-launching configurations to use for AMI distribution. Detailed below.
    #[builder(into)]
    #[serde(rename = "fastLaunchConfigurations")]
    pub r#fast_launch_configurations: Option<Vec<super::super::types::imagebuilder::DistributionConfigurationDistributionFastLaunchConfiguration>>,
    /// Set of launch template configuration settings that apply to image distribution. Detailed below.
    #[builder(into)]
    #[serde(rename = "launchTemplateConfigurations")]
    pub r#launch_template_configurations: Option<Vec<super::super::types::imagebuilder::DistributionConfigurationDistributionLaunchTemplateConfiguration>>,
    /// Set of Amazon Resource Names (ARNs) of License Manager License Configurations.
    #[builder(into)]
    #[serde(rename = "licenseConfigurationArns")]
    pub r#license_configuration_arns: Option<Vec<String>>,
    /// AWS Region for the distribution.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
    /// Configuration block with S3 export settings. Detailed below.
    #[builder(into)]
    #[serde(rename = "s3ExportConfiguration")]
    pub r#s_3_export_configuration: Option<Box<super::super::types::imagebuilder::DistributionConfigurationDistributionS3ExportConfiguration>>,
}
