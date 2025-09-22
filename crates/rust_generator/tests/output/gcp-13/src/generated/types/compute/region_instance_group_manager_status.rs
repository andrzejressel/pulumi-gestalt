#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionInstanceGroupManagerStatus {
    /// Properties to set on all instances in the group. After setting
    /// allInstancesConfig on the group, you must update the group's instances to
    /// apply the configuration.
    #[builder(into)]
    #[serde(rename = "allInstancesConfigs")]
    pub r#all_instances_configs: Option<Vec<super::super::types::compute::RegionInstanceGroupManagerStatusAllInstancesConfig>>,
    /// A bit indicating whether the managed instance group is in a stable state. A stable state means that: none of the instances in the managed instance group is currently undergoing any type of change (for example, creation, restart, or deletion); no future changes are scheduled for instances in the managed instance group; and the managed instance group itself is not being modified.
    #[builder(into)]
    #[serde(rename = "isStable")]
    pub r#is_stable: Option<bool>,
    /// Stateful status of the given Instance Group Manager.
    #[builder(into)]
    #[serde(rename = "statefuls")]
    pub r#statefuls: Option<Vec<super::super::types::compute::RegionInstanceGroupManagerStatusStateful>>,
    /// A bit indicating whether version target has been reached in this managed instance group, i.e. all instances are in their target version. Instances' target version are specified by version field on Instance Group Manager.
    #[builder(into)]
    #[serde(rename = "versionTargets")]
    pub r#version_targets: Option<Vec<super::super::types::compute::RegionInstanceGroupManagerStatusVersionTarget>>,
}
