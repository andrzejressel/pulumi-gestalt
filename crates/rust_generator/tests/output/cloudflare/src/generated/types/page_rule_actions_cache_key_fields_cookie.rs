#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PageRuleActionsCacheKeyFieldsCookie {
    /// Check for presence of specified cookies, without including their actual values.
    #[builder(into)]
    #[serde(rename = "checkPresences")]
    pub r#check_presences: Option<Vec<String>>,
    /// Use values of specified cookies in Cache Key.
    #[builder(into)]
    #[serde(rename = "includes")]
    pub r#includes: Option<Vec<String>>,
}
