#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceTemplateAdvancedMachineFeatures {
    /// Defines whether the instance should have nested virtualization enabled. Defaults to false.
    #[builder(into)]
    #[serde(rename = "enableNestedVirtualization")]
    pub r#enable_nested_virtualization: Option<bool>,
    /// Whether to enable UEFI networking for instance creation.
    #[builder(into)]
    #[serde(rename = "enableUefiNetworking")]
    pub r#enable_uefi_networking: Option<bool>,
    /// [The PMU](https://cloud.google.com/compute/docs/pmu-overview) is a hardware component within the CPU core that monitors how the processor runs code. Valid values for the level of PMU are `STANDARD`, `ENHANCED`, and `ARCHITECTURAL`.
    #[builder(into)]
    #[serde(rename = "performanceMonitoringUnit")]
    pub r#performance_monitoring_unit: Option<String>,
    /// The number of threads per physical core. To disable [simultaneous multithreading (SMT)](https://cloud.google.com/compute/docs/instances/disabling-smt) set this to 1.
    #[builder(into)]
    #[serde(rename = "threadsPerCore")]
    pub r#threads_per_core: Option<i32>,
    /// Turbo frequency mode to use for the instance. Supported modes are currently either `ALL_CORE_MAX` or unset (default).
    #[builder(into)]
    #[serde(rename = "turboMode")]
    pub r#turbo_mode: Option<String>,
    /// The number of physical cores to expose to an instance. [visible cores info (VC)](https://cloud.google.com/compute/docs/instances/customize-visible-cores).
    #[builder(into)]
    #[serde(rename = "visibleCoreCount")]
    pub r#visible_core_count: Option<i32>,
}
