#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MetadataSupport {
    /// The email address of the support contact.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// The link for support help.
    #[builder(into)]
    #[serde(rename = "link")]
    pub r#link: Option<String>,
    /// The name of the support contact.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The type of support for content item. Possible values are `Microsoft`, `Partner` and `Community`.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: String,
}
