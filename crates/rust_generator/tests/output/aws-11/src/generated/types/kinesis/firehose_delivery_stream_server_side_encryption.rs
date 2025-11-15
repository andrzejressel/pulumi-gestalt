#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirehoseDeliveryStreamServerSideEncryption {
    /// Whether to enable encryption at rest. Default is `false`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Amazon Resource Name (ARN) of the encryption key. Required when `key_type` is `CUSTOMER_MANAGED_CMK`.
    #[builder(into)]
    #[serde(rename = "keyArn")]
    pub r#key_arn: Option<String>,
    /// Type of encryption key. Default is `AWS_OWNED_CMK`. Valid values are `AWS_OWNED_CMK` and `CUSTOMER_MANAGED_CMK`
    #[builder(into)]
    #[serde(rename = "keyType")]
    pub r#key_type: Option<String>,
}
