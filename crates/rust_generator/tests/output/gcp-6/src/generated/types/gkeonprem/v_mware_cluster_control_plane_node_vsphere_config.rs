#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VMwareClusterControlPlaneNodeVsphereConfig {
    /// The Vsphere datastore used by the Control Plane Node.
    #[builder(into, default)]
    #[serde(rename = "datastore")]
    pub r#datastore: Box<Option<String>>,
    /// The Vsphere storage policy used by the control plane Node.
    #[builder(into, default)]
    #[serde(rename = "storagePolicyName")]
    pub r#storage_policy_name: Box<Option<String>>,
}
