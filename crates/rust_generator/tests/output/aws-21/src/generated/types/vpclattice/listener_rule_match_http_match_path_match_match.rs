#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ListenerRuleMatchHttpMatchPathMatchMatch {
    /// Specifies an exact type match.
    #[builder(into)]
    #[serde(rename = "exact")]
    pub r#exact: Option<String>,
    /// Specifies a prefix type match. Matches the value with the prefix.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<String>,
}
