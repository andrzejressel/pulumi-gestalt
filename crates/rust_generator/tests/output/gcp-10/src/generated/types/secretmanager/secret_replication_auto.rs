#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SecretReplicationAuto {
    /// The customer-managed encryption configuration of the Secret.
    /// If no configuration is provided, Google-managed default
    /// encryption is used.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customerManagedEncryption")]
    pub r#customer_managed_encryption: Option<Box<super::super::types::secretmanager::SecretReplicationAutoCustomerManagedEncryption>>,
}
