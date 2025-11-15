#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareClusterVcenter {
    /// (Output)
    /// The vCenter IP address.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    /// Contains the vCenter CA certificate public key for SSL verification.
    #[builder(into)]
    #[serde(rename = "caCertData")]
    pub r#ca_cert_data: Option<String>,
    /// The name of the vCenter cluster for the user cluster.
    #[builder(into)]
    #[serde(rename = "cluster")]
    pub r#cluster: Option<String>,
    /// The name of the vCenter datacenter for the user cluster.
    #[builder(into)]
    #[serde(rename = "datacenter")]
    pub r#datacenter: Option<String>,
    /// The name of the vCenter datastore for the user cluster.
    #[builder(into)]
    #[serde(rename = "datastore")]
    pub r#datastore: Option<String>,
    /// The name of the vCenter folder for the user cluster.
    #[builder(into)]
    #[serde(rename = "folder")]
    pub r#folder: Option<String>,
    /// The name of the vCenter resource pool for the user cluster.
    #[builder(into)]
    #[serde(rename = "resourcePool")]
    pub r#resource_pool: Option<String>,
    /// The name of the vCenter storage policy for the user cluster.
    #[builder(into)]
    #[serde(rename = "storagePolicyName")]
    pub r#storage_policy_name: Option<String>,
}
