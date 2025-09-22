#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceAddOn {
    /// The daily time when an automatic snapshot will be created. Must be in HH:00 format, and in an hourly increment and specified in Coordinated Universal Time (UTC). The snapshot will be automatically created between the time specified and up to 45 minutes after.
    #[builder(into)]
    #[serde(rename = "snapshotTime")]
    pub r#snapshot_time: String,
    /// The status of the add on. Valid Values: `Enabled`, `Disabled`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
    /// The add-on type. There is currently only one valid type `AutoSnapshot`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
