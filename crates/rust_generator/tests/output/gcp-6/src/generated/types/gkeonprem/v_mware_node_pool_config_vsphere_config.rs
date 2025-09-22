#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VMwareNodePoolConfigVsphereConfig {
    /// The name of the vCenter datastore. Inherited from the user cluster.
    #[builder(into)]
    #[serde(rename = "datastore")]
    pub r#datastore: Option<String>,
    /// Vsphere host groups to apply to all VMs in the node pool
    #[builder(into)]
    #[serde(rename = "hostGroups")]
    pub r#host_groups: Option<Vec<String>>,
    /// Tags to apply to VMs.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Option<Vec<super::super::types::gkeonprem::VMwareNodePoolConfigVsphereConfigTag>>,
}
