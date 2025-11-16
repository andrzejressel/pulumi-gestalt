#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNodeSpecListenerTimeoutTcpIdle {
    /// Unit of time. Valid values: `ms`, `s`.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: String,
    /// Number of time units. Minimum value of `0`.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: i32,
}
