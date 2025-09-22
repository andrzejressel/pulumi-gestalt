#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetFlexibleServerStorage {
    /// Is Storage Auto Grow enabled?
    #[builder(into)]
    #[serde(rename = "autoGrowEnabled")]
    pub r#auto_grow_enabled: bool,
    /// Should IOPS be scaled automatically?
    #[builder(into)]
    #[serde(rename = "ioScalingEnabled")]
    pub r#io_scaling_enabled: bool,
    /// The storage IOPS of the MySQL Flexible Server.
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: i32,
    /// The max storage allowed for the MySQL Flexible Server.
    #[builder(into)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: i32,
}
