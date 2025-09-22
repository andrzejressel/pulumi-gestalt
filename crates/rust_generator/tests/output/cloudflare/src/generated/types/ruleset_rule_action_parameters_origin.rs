#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RulesetRuleActionParametersOrigin {
    /// Origin Hostname where request is sent.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Option<String>,
    /// Origin Port where request is sent.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
}
