#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreferenceSetVirtualMachinePreferencesComputeEnginePreferencesMachinePreferences {
    /// Compute Engine machine series to consider for insights and recommendations. If empty, no restriction is applied on the machine series.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "allowedMachineSeries")]
    pub r#allowed_machine_series: Option<Vec<super::super::types::migrationcenter::PreferenceSetVirtualMachinePreferencesComputeEnginePreferencesMachinePreferencesAllowedMachineSeries>>,
}
