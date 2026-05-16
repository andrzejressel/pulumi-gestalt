#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFirewallEncryptionConfiguration {
    /// The ID of the AWS Key Management Service (AWS KMS) customer managed key.
    #[builder(into)]
    #[serde(rename = "keyId")]
    pub r#key_id: String,
    /// The type of the AWS Key Management Service (AWS KMS) key use by the firewall.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
