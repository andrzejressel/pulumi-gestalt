#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpaceSpaceSettingsJupyterServerAppSettingsDefaultResourceSpec {
    /// The instance type.
    #[builder(into)]
    #[serde(rename = "instanceType")]
    pub r#instance_type: Option<String>,
    /// The Amazon Resource Name (ARN) of the Lifecycle Configuration attached to the Resource.
    #[builder(into)]
    #[serde(rename = "lifecycleConfigArn")]
    pub r#lifecycle_config_arn: Option<String>,
    /// The Amazon Resource Name (ARN) of the SageMaker image created on the instance.
    #[builder(into)]
    #[serde(rename = "sagemakerImageArn")]
    pub r#sagemaker_image_arn: Option<String>,
    /// The SageMaker Image Version Alias.
    #[builder(into)]
    #[serde(rename = "sagemakerImageVersionAlias")]
    pub r#sagemaker_image_version_alias: Option<String>,
    /// The ARN of the image version created on the instance.
    #[builder(into)]
    #[serde(rename = "sagemakerImageVersionArn")]
    pub r#sagemaker_image_version_arn: Option<String>,
}
