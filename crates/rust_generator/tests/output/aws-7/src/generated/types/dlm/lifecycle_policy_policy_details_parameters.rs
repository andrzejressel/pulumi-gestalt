#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LifecyclePolicyPolicyDetailsParameters {
    /// Indicates whether to exclude the root volume from snapshots created using CreateSnapshots. The default is `false`.
    #[builder(into)]
    #[serde(rename = "excludeBootVolume")]
    pub r#exclude_boot_volume: Option<bool>,
    /// Applies to AMI lifecycle policies only. Indicates whether targeted instances are rebooted when the lifecycle policy runs. `true` indicates that targeted instances are not rebooted when the policy runs. `false` indicates that target instances are rebooted when the policy runs. The default is `true` (instances are not rebooted).
    #[builder(into)]
    #[serde(rename = "noReboot")]
    pub r#no_reboot: Option<bool>,
}
