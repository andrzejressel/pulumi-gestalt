#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OntapStorageVirtualMachineEndpointManagement {
    /// The Domain Name Service (DNS) name for the storage virtual machine. You can mount your storage virtual machine using its DNS name.
    #[builder(into)]
    #[serde(rename = "dnsName")]
    pub r#dns_name: Option<String>,
    /// IP addresses of the storage virtual machine endpoint.
    #[builder(into)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Option<Vec<String>>,
}
