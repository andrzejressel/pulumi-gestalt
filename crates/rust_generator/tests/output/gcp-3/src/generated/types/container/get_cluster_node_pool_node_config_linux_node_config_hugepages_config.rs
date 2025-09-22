#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterNodePoolNodeConfigLinuxNodeConfigHugepagesConfig {
    /// Amount of 1G hugepages.
    #[builder(into)]
    #[serde(rename = "hugepageSize1g")]
    pub r#hugepage_size_1_g: i32,
    /// Amount of 2M hugepages.
    #[builder(into)]
    #[serde(rename = "hugepageSize2m")]
    pub r#hugepage_size_2_m: i32,
}
