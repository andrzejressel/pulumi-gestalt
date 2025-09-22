#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesDatabaseServer {
    /// The full name for the availability set. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "availabilitySetName")]
    pub r#availability_set_name: Option<String>,
    /// A `load_balancer` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "loadBalancer")]
    pub r#load_balancer: Option<Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesDatabaseServerLoadBalancer>>,
    /// One or more `virtual_machine` blocks as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "virtualMachines")]
    pub r#virtual_machines: Option<Vec<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNamesDatabaseServerVirtualMachine>>,
}
