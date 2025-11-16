#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkteamWorkerAccessConfigurationS3Presign {
    /// Use this parameter to specify the allowed request source. Possible sources are either SourceIp or VpcSourceIp. see IAM Policy Constraints details below.
    #[builder(into)]
    #[serde(rename = "iamPolicyConstraints")]
    pub r#iam_policy_constraints: Option<Box<super::super::types::sagemaker::WorkteamWorkerAccessConfigurationS3PresignIamPolicyConstraints>>,
}
