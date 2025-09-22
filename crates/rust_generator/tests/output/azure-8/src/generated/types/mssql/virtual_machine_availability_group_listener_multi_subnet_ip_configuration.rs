#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualMachineAvailabilityGroupListenerMultiSubnetIpConfiguration {
    /// The private IP Address of the listener. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: String,
    /// The ID of the Sql Virtual Machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sqlVirtualMachineId")]
    pub r#sql_virtual_machine_id: String,
    /// The ID of the Subnet to create the listener. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** `sql_virtual_machine_id` should match with the SQL Virtual Machines specified in `replica`.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}
