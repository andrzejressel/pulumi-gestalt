#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSinkExclusion {
    /// A description of this exclusion.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    /// Whether this exclusion is disabled and it does not exclude any log entries.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: bool,
    /// An advanced logs filter that matches the log entries to be excluded.
    #[builder(into)]
    #[serde(rename = "filter")]
    pub r#filter: String,
    /// A client-assigned identifier, such as `load-balancer-exclusion`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
