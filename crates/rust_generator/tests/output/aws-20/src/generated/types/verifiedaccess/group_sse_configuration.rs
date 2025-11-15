#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GroupSseConfiguration {
    /// Boolean flag to indicate that the CMK should be used.
    #[builder(into)]
    #[serde(rename = "customerManagedKeyEnabled")]
    pub r#customer_managed_key_enabled: Option<bool>,
    /// ARN of the KMS key to use.
    #[builder(into)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Option<String>,
}
