#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessGroupIncludeSaml {
    /// The name of the SAML attribute.
    #[builder(into)]
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Option<String>,
    /// The SAML attribute value to look for.
    #[builder(into)]
    #[serde(rename = "attributeValue")]
    pub r#attribute_value: Option<String>,
    /// The ID of your SAML identity provider.
    #[builder(into)]
    #[serde(rename = "identityProviderId")]
    pub r#identity_provider_id: Option<String>,
}
