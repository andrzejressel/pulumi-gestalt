#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustAccessApplicationSaasAppCustomAttributeSource {
    /// The name of the attribute as provided by the IDP.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A mapping from IdP ID to claim name.
    #[builder(into)]
    #[serde(rename = "nameByIdp")]
    pub r#name_by_idp: Option<std::collections::HashMap<String, String>>,
}
