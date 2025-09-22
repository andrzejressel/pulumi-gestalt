#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserProfileUserSettingsCanvasAppSettingsEmrServerlessSettings {
    /// The Amazon Resource Name (ARN) of the AWS IAM role that is assumed for running Amazon EMR Serverless jobs in SageMaker Canvas. This role should have the necessary permissions to read and write data attached and a trust relationship with EMR Serverless.
    #[builder(into)]
    #[serde(rename = "executionRoleArn")]
    pub r#execution_role_arn: Option<String>,
    /// Describes whether Amazon EMR Serverless job capabilities are enabled or disabled in the SageMaker Canvas application. Valid values are: `ENABLED` and `DISABLED`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}
