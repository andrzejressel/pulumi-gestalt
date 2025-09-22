#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreferenceSetVirtualMachinePreferences {
    /// Commitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: `COMMITMENT_PLAN_UNSPECIFIED`, `COMMITMENT_PLAN_NONE`, `COMMITMENT_PLAN_ONE_YEAR`, `COMMITMENT_PLAN_THREE_YEARS`
    #[builder(into)]
    #[serde(rename = "commitmentPlan")]
    pub r#commitment_plan: Option<String>,
    /// The user preferences relating to Compute Engine target platform.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "computeEnginePreferences")]
    pub r#compute_engine_preferences: Option<Box<super::super::types::migrationcenter::PreferenceSetVirtualMachinePreferencesComputeEnginePreferences>>,
    /// The user preferences relating to target regions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "regionPreferences")]
    pub r#region_preferences: Option<Box<super::super::types::migrationcenter::PreferenceSetVirtualMachinePreferencesRegionPreferences>>,
    /// Sizing optimization strategy specifies the preferred strategy used when extrapolating usage data to calculate insights and recommendations for a virtual machine. If you are unsure which value to set, a moderate sizing optimization strategy is often a good value to start with. Possible values: `SIZING_OPTIMIZATION_STRATEGY_UNSPECIFIED`, `SIZING_OPTIMIZATION_STRATEGY_SAME_AS_SOURCE`, `SIZING_OPTIMIZATION_STRATEGY_MODERATE`, `SIZING_OPTIMIZATION_STRATEGY_AGGRESSIVE`
    #[builder(into)]
    #[serde(rename = "sizingOptimizationStrategy")]
    pub r#sizing_optimization_strategy: Option<String>,
    /// Preferences concerning Sole Tenancy nodes and VMs.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "soleTenancyPreferences")]
    pub r#sole_tenancy_preferences: Option<Box<super::super::types::migrationcenter::PreferenceSetVirtualMachinePreferencesSoleTenancyPreferences>>,
    /// Target product for assets using this preference set. Specify either target product or business goal, but not both. Possible values: `COMPUTE_MIGRATION_TARGET_PRODUCT_UNSPECIFIED`, `COMPUTE_MIGRATION_TARGET_PRODUCT_COMPUTE_ENGINE`, `COMPUTE_MIGRATION_TARGET_PRODUCT_VMWARE_ENGINE`, `COMPUTE_MIGRATION_TARGET_PRODUCT_SOLE_TENANCY`
    #[builder(into)]
    #[serde(rename = "targetProduct")]
    pub r#target_product: Option<String>,
    /// The user preferences relating to Google Cloud VMware Engine target platform.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vmwareEnginePreferences")]
    pub r#vmware_engine_preferences: Option<Box<super::super::types::migrationcenter::PreferenceSetVirtualMachinePreferencesVmwareEnginePreferences>>,
}
