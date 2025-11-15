#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FlexibleServerStorage {
    /// Should Storage Auto Grow be enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "autoGrowEnabled")]
    pub r#auto_grow_enabled: Option<bool>,
    /// Should IOPS be scaled automatically? If `true`, `iops` can not be set. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "ioScalingEnabled")]
    pub r#io_scaling_enabled: Option<bool>,
    /// The storage IOPS for the MySQL Flexible Server. Possible values are between `360` and `20000`.
    #[builder(into)]
    #[serde(rename = "iops")]
    pub r#iops: Option<i32>,
    /// The max storage allowed for the MySQL Flexible Server. Possible values are between `20` and `16384`.
    /// 
    /// > **Note:** Decreasing `size_gb` forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sizeGb")]
    pub r#size_gb: Option<i32>,
}
