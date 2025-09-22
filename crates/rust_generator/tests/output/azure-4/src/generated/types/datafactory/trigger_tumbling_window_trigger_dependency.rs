#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TriggerTumblingWindowTriggerDependency {
    /// The offset of the dependency trigger. Must be in Timespan format (±hh:mm:ss) and must be a negative offset for a self dependency.
    #[builder(into)]
    #[serde(rename = "offset")]
    pub r#offset: Option<String>,
    /// The size of the dependency tumbling window. Must be in Timespan format (hh:mm:ss).
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Option<String>,
    /// The dependency trigger name. If not specified, it will use self dependency.
    #[builder(into)]
    #[serde(rename = "triggerName")]
    pub r#trigger_name: Option<String>,
}
