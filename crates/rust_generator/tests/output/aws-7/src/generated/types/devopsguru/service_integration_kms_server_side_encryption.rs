#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceIntegrationKmsServerSideEncryption {
    /// KMS key ID. This value can be a key ID, key ARN, alias name, or alias ARN.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// Specifies whether KMS integration is enabled. Valid values are `DISABLED` and `ENABLED`.
    #[builder(into)]
    #[serde(rename = "optInStatus")]
    pub r#opt_in_status: Option<String>,
    /// Type of KMS key used. Valid values are `CUSTOMER_MANAGED_KEY` and `AWS_OWNED_KMS_KEY`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
