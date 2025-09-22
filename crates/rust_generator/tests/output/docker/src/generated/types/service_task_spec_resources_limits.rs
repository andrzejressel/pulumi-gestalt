#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTaskSpecResourcesLimits {
    /// The amounf of memory in bytes the container allocates
    #[builder(into)]
    #[serde(rename = "memoryBytes")]
    pub r#memory_bytes: Option<i32>,
    /// CPU shares in units of `1/1e9` (or `10^-9`) of the CPU. Should be at least `1000000`
    #[builder(into)]
    #[serde(rename = "nanoCpus")]
    pub r#nano_cpus: Option<i32>,
}
