#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterNodeConfigLocalNvmeSsdBlockConfig {
    /// Number of raw-block local NVMe SSD disks to be attached to the node. Each local SSD is 375 GB in size.
    #[builder(into)]
    #[serde(rename = "localSsdCount")]
    pub r#local_ssd_count: Box<i32>,
}
