#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NodeTemplateDisk {
    /// Specifies the number of such disks.
    #[builder(into)]
    #[serde(rename = "diskCount")]
    pub r#disk_count: Option<i32>,
    /// Specifies the size of the disk in base-2 GB.
    #[builder(into)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Option<i32>,
    /// Specifies the desired disk type on the node. This disk type must be a local storage type (e.g.: local-ssd). Note that for nodeTemplates, this should be the name of the disk type and not its URL.
    #[builder(into)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Option<String>,
}
