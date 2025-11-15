#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EmailServiceDomainVerificationRecordDkim2 {
    /// The name of the Email Communication Service resource. If `domain_management` is `AzureManaged`, the name must be `AzureManagedDomain`. Changing this forces a new Email Communication Service to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Represents an expiry time in seconds to represent how long this entry can be cached by the resolver, default = 3600sec.
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: Option<i32>,
    /// Type of the DNS record. Example: TXT
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    /// Value of the DNS record.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
