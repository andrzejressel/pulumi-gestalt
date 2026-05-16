#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PolicyVmWorkloadSettings {
    /// The compression setting for the VM Workload Backup Policy. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "compressionEnabled")]
    pub r#compression_enabled: Option<bool>,
    /// The timezone for the VM Workload Backup Policy. [The possible values are defined here](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/).
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: String,
}
