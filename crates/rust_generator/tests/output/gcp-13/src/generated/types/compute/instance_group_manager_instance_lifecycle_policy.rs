#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceGroupManagerInstanceLifecyclePolicy {
    /// , Default behavior for all instance or health check failures. Valid options are: `REPAIR`, `DO_NOTHING`. If `DO_NOTHING` then instances will not be repaired. If `REPAIR` (default), then failed instances will be repaired.
    /// - - -
    #[builder(into)]
    #[serde(rename = "defaultActionOnFailure")]
    pub r#default_action_on_failure: Option<String>,
    /// , Specifies whether to apply the group's latest configuration when repairing a VM. Valid options are: `YES`, `NO`. If `YES` and you updated the group's instance template or per-instance configurations after the VM was created, then these changes are applied when VM is repaired. If `NO` (default), then updates are applied in accordance with the group's update policy type.
    #[builder(into)]
    #[serde(rename = "forceUpdateOnRepair")]
    pub r#force_update_on_repair: Option<String>,
}
