#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerVolumeEfsVolumeConfigurationAuthorizationConfig {
    /// The Amazon EFS access point ID to use.
    #[builder(into)]
    #[serde(rename = "accessPointId")]
    pub r#access_point_id: Box<String>,
    /// Whether or not to use the AWS Batch job IAM role defined in a job definition when mounting the Amazon EFS file system.
    #[builder(into)]
    #[serde(rename = "iam")]
    pub r#iam: Box<String>,
}
