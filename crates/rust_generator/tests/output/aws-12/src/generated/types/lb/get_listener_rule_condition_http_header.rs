#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetListenerRuleConditionHttpHeader {
    /// Name of the HTTP header to match.
    #[builder(into)]
    #[serde(rename = "httpHeaderName")]
    pub r#http_header_name: String,
    /// Set of `key`-`value` pairs indicating the query string parameters to match.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
