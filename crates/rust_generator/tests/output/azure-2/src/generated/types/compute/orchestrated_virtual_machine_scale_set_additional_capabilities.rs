#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OrchestratedVirtualMachineScaleSetAdditionalCapabilities {
    /// Should the capacity to enable Data Disks of the `UltraSSD_LRS` storage account type be supported on this Virtual Machine Scale Set? Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "ultraSsdEnabled")]
    pub r#ultra_ssd_enabled: Box<Option<bool>>,
}
