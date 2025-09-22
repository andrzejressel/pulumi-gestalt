#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FlexibleAppVersionResources {
    /// Number of CPU cores needed.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: Option<i32>,
    /// Disk size (GB) needed.
    #[builder(into)]
    #[serde(rename = "diskGb")]
    pub r#disk_gb: Option<i32>,
    /// Memory (GB) needed.
    #[builder(into)]
    #[serde(rename = "memoryGb")]
    pub r#memory_gb: Option<f64>,
    /// List of ports, or port pairs, to forward from the virtual machine to the application container.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "volumes")]
    pub r#volumes: Option<Vec<super::super::types::appengine::FlexibleAppVersionResourcesVolume>>,
}
