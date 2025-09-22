#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagedInstanceFailoverGroupReadWriteEndpointFailoverPolicy {
    /// Applies only if `mode` is `Automatic`. The grace period in minutes before failover with data loss is attempted.
    #[builder(into)]
    #[serde(rename = "graceMinutes")]
    pub r#grace_minutes: Option<i32>,
    /// The failover mode. Possible values are `Automatic` or `Manual`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
}
