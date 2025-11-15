#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreferenceSetVirtualMachinePreferencesComputeEnginePreferencesMachinePreferencesAllowedMachineSeries {
    /// Code to identify a Compute Engine machine series. Consult https://cloud.google.com/compute/docs/machine-resource#machine_type_comparison for more details on the available series.
    #[builder(into)]
    #[serde(rename = "code")]
    pub r#code: Option<String>,
}
