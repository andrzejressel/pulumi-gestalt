#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterMaintenanceEvent {
    /// (Output)
    /// The time when the maintenance event request was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Option<String>,
    /// (Output)
    /// The time when the maintenance event ended, either successfully or not. If
    /// the maintenance event is split into multiple maintenance windows,
    /// end_time is only updated when the whole flow ends.
    #[builder(into)]
    #[serde(rename = "endTime")]
    pub r#end_time: Option<String>,
    /// (Output)
    /// The operation for running the maintenance event. Specified in the format
    /// projects/*/locations/*/operations/*. If the maintenance event is split
    /// into multiple operations (e.g. due to maintenance windows), the latest
    /// one is recorded.
    #[builder(into)]
    #[serde(rename = "operation")]
    pub r#operation: Option<String>,
    /// (Output)
    /// The schedule of the maintenance event.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Option<String>,
    /// (Output)
    /// The time when the maintenance event started.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Option<String>,
    /// (Output)
    /// Indicates the maintenance event state.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// (Output)
    /// The target version of the cluster.
    #[builder(into)]
    #[serde(rename = "targetVersion")]
    pub r#target_version: Option<String>,
    /// (Output)
    /// Indicates the maintenance event type.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    /// (Output)
    /// The time when the maintenance event message was updated.
    #[builder(into)]
    #[serde(rename = "updateTime")]
    pub r#update_time: Option<String>,
    /// (Output)
    /// UUID of the maintenance event.
    #[builder(into)]
    #[serde(rename = "uuid")]
    pub r#uuid: Option<String>,
}
