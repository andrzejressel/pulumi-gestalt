#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IntegrationAccountAgreementHostIdentity {
    /// The authenticating body that provides unique host identities to organizations.
    #[builder(into)]
    #[serde(rename = "qualifier")]
    pub r#qualifier: String,
    /// The value that identifies the documents that your logic apps receive.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
