#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderFeaturesVirtualMachineScaleSet {
    #[builder(into)]
    #[serde(rename = "forceDelete")]
    pub r#force_delete: Option<bool>,
    #[builder(into)]
    #[serde(rename = "reimageOnManualUpgrade")]
    pub r#reimage_on_manual_upgrade: Option<bool>,
    #[builder(into)]
    #[serde(rename = "rollInstancesWhenRequired")]
    pub r#roll_instances_when_required: Option<bool>,
    #[builder(into)]
    #[serde(rename = "scaleToZeroBeforeDeletion")]
    pub r#scale_to_zero_before_deletion: Option<bool>,
}
