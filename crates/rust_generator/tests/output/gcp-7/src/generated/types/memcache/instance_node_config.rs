#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceNodeConfig {
    /// Number of CPUs per node.
    #[builder(into)]
    #[serde(rename = "cpuCount")]
    pub r#cpu_count: i32,
    /// Memory size in Mebibytes for each memcache node.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "memorySizeMb")]
    pub r#memory_size_mb: i32,
}
