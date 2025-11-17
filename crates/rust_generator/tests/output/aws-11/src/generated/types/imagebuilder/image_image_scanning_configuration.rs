#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ImageImageScanningConfiguration {
    /// Configuration block with ECR configuration. Detailed below.
    #[builder(into)]
    #[serde(rename = "ecrConfiguration")]
    pub r#ecr_configuration: Option<Box<super::super::types::imagebuilder::ImageImageScanningConfigurationEcrConfiguration>>,
    /// Indicates whether Image Builder keeps a snapshot of the vulnerability scans that Amazon Inspector runs against the build instance when you create a new image. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "imageScanningEnabled")]
    pub r#image_scanning_enabled: Option<bool>,
}
