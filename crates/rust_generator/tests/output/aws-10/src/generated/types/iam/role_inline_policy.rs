#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RoleInlinePolicy {
    /// Name of the role policy.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Policy document as a JSON formatted string.
    #[builder(into)]
    #[serde(rename = "policy")]
    pub r#policy: Option<String>,
}
