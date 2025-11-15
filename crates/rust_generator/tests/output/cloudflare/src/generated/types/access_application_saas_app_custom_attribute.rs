#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessApplicationSaasAppCustomAttribute {
    /// A friendly name for the attribute as provided to the SaaS app.
    #[builder(into)]
    #[serde(rename = "friendlyName")]
    pub r#friendly_name: Option<String>,
    /// The name of the attribute as provided to the SaaS app.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// A globally unique name for an identity or service provider.
    #[builder(into)]
    #[serde(rename = "nameFormat")]
    pub r#name_format: Option<String>,
    /// True if the attribute must be always present.
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: Option<bool>,
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<super::types::AccessApplicationSaasAppCustomAttributeSource>,
}
