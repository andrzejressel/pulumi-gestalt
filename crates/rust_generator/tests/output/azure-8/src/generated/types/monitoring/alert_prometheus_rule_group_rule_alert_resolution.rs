#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AlertPrometheusRuleGroupRuleAlertResolution {
    /// Is the alert auto-resolution? Possible values are `true` and `false`.
    #[builder(into)]
    #[serde(rename = "autoResolved")]
    pub r#auto_resolved: Option<bool>,
    /// Specifies the alert auto-resolution interval, represented in ISO 8601 duration format.
    #[builder(into)]
    #[serde(rename = "timeToResolve")]
    pub r#time_to_resolve: Option<String>,
}
