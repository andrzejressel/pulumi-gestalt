#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagedClusterNodeType {
    /// Sets the port range available for applications. Format is `<from_port>-<to_port>`, for example `10000-20000`.
    #[builder(into)]
    #[serde(rename = "applicationPortRange")]
    pub r#application_port_range: String,
    /// Specifies a list of key/value pairs used to set capacity tags for this node type.
    #[builder(into)]
    #[serde(rename = "capacities")]
    pub r#capacities: Option<std::collections::HashMap<String, String>>,
    /// The size of the data disk in gigabytes..
    #[builder(into)]
    #[serde(rename = "dataDiskSizeGb")]
    pub r#data_disk_size_gb: i32,
    /// The type of the disk to use for storing data. It can be one of `Premium_LRS`, `Standard_LRS`, or `StandardSSD_LRS`. Defaults to `Standard_LRS`.
    #[builder(into)]
    #[serde(rename = "dataDiskType")]
    pub r#data_disk_type: Option<String>,
    /// Sets the port range available for the OS. Format is `<from_port>-<to_port>`, for example `10000-20000`. There has to be at least 255 ports available and cannot overlap with `application_port_range`..
    #[builder(into)]
    #[serde(rename = "ephemeralPortRange")]
    pub r#ephemeral_port_range: String,
    /// The ID of the Resource Group.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// If set the node type can be composed of multiple placement groups.
    #[builder(into)]
    #[serde(rename = "multiplePlacementGroupsEnabled")]
    pub r#multiple_placement_groups_enabled: Option<bool>,
    /// The name which should be used for this node type.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Specifies a list of placement tags that can be used to indicate where services should run..
    #[builder(into)]
    #[serde(rename = "placementProperties")]
    pub r#placement_properties: Option<std::collections::HashMap<String, String>>,
    /// If set to true, system services will run on this node type. Only one node type should be marked as primary. Primary node type cannot be deleted or changed once they're created.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Option<bool>,
    /// If set to true, only stateless workloads can run on this node type.
    #[builder(into)]
    #[serde(rename = "stateless")]
    pub r#stateless: Option<bool>,
    /// The offer type of the marketplace image cluster VMs will use.
    #[builder(into)]
    #[serde(rename = "vmImageOffer")]
    pub r#vm_image_offer: String,
    /// The publisher of the marketplace image cluster VMs will use.
    #[builder(into)]
    #[serde(rename = "vmImagePublisher")]
    pub r#vm_image_publisher: String,
    /// The SKU of the marketplace image cluster VMs will use.
    #[builder(into)]
    #[serde(rename = "vmImageSku")]
    pub r#vm_image_sku: String,
    /// The version of the marketplace image cluster VMs will use.
    #[builder(into)]
    #[serde(rename = "vmImageVersion")]
    pub r#vm_image_version: String,
    /// The number of instances this node type will launch.
    #[builder(into)]
    #[serde(rename = "vmInstanceCount")]
    pub r#vm_instance_count: i32,
    /// One or more `vm_secrets` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "vmSecrets")]
    pub r#vm_secrets: Option<Vec<super::super::types::servicefabric::ManagedClusterNodeTypeVmSecret>>,
    /// The size of the instances in this node type.
    #[builder(into)]
    #[serde(rename = "vmSize")]
    pub r#vm_size: String,
}
