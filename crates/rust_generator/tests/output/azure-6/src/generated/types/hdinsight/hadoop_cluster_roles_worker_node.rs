#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HadoopClusterRolesWorkerNode {
    /// A `autoscale` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "autoscale")]
    pub r#autoscale: Box<Option<super::super::types::hdinsight::HadoopClusterRolesWorkerNodeAutoscale>>,
    /// The Password associated with the local administrator for the Worker Nodes. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** If specified, this password must be at least 10 characters in length and must contain at least one digit, one uppercase and one lower case letter, one non-alphanumeric character (except characters ' " ` \).
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// The script action which will run on the cluster. One or more `script_actions` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "scriptActions")]
    pub r#script_actions: Box<Option<Vec<super::super::types::hdinsight::HadoopClusterRolesWorkerNodeScriptAction>>>,
    /// A list of SSH Keys which should be used for the local administrator on the Worker Nodes. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** Either a `password` or one or more `ssh_keys` must be specified - but not both.
    #[builder(into, default)]
    #[serde(rename = "sshKeys")]
    pub r#ssh_keys: Box<Option<Vec<String>>>,
    /// The ID of the Subnet within the Virtual Network where the Worker Nodes should be provisioned within. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<Option<String>>,
    /// The number of instances which should be run for the Worker Nodes.
    #[builder(into)]
    #[serde(rename = "targetInstanceCount")]
    pub r#target_instance_count: Box<i32>,
    /// The Username of the local administrator for the Worker Nodes. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
    /// The ID of the Virtual Network where the Worker Nodes should be provisioned within. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "virtualNetworkId")]
    pub r#virtual_network_id: Box<Option<String>>,
    /// The Size of the Virtual Machine which should be used as the Worker Nodes. Possible values are `ExtraSmall`, `Small`, `Medium`, `Large`, `ExtraLarge`, `A5`, `A6`, `A7`, `A8`, `A9`, `A10`, `A11`, `Standard_A1_V2`, `Standard_A2_V2`, `Standard_A2m_V2`, `Standard_A3`, `Standard_A4_V2`, `Standard_A4m_V2`, `Standard_A8_V2`, `Standard_A8m_V2`, `Standard_D1`, `Standard_D2`, `Standard_D3`, `Standard_D4`, `Standard_D11`, `Standard_D12`, `Standard_D13`, `Standard_D14`, `Standard_D1_V2`, `Standard_D2_V2`, `Standard_D3_V2`, `Standard_D4_V2`, `Standard_D5_V2`, `Standard_D11_V2`, `Standard_D12_V2`, `Standard_D13_V2`, `Standard_D14_V2`, `Standard_DS1_V2`, `Standard_DS2_V2`, `Standard_DS3_V2`, `Standard_DS4_V2`, `Standard_DS5_V2`, `Standard_DS11_V2`, `Standard_DS12_V2`, `Standard_DS13_V2`, `Standard_DS14_V2`, `Standard_E2_V3`, `Standard_E4_V3`, `Standard_E8_V3`, `Standard_E16_V3`, `Standard_E20_V3`, `Standard_E32_V3`, `Standard_E64_V3`, `Standard_E64i_V3`, `Standard_E2s_V3`, `Standard_E4s_V3`, `Standard_E8s_V3`, `Standard_E16s_V3`, `Standard_E20s_V3`, `Standard_E32s_V3`, `Standard_E64s_V3`, `Standard_E64is_V3`, `Standard_D2a_V4`, `Standard_D4a_V4`, `Standard_D8a_V4`, `Standard_D16a_V4`, `Standard_D32a_V4`, `Standard_D48a_V4`, `Standard_D64a_V4`, `Standard_D96a_V4`, `Standard_E2a_V4`, `Standard_E4a_V4`, `Standard_E8a_V4`, `Standard_E16a_V4`, `Standard_E20a_V4`, `Standard_E32a_V4`, `Standard_E48a_V4`, `Standard_E64a_V4`, `Standard_E96a_V4`, `Standard_D2ads_V5`, `Standard_D4ads_V5`, `Standard_D8ads_V5`, `Standard_D16ads_V5`, `Standard_D32ads_V5`, `Standard_D48ads_V5`, `Standard_D64ads_V5`, `Standard_D96ads_V5`, `Standard_E2ads_V5`, `Standard_E4ads_V5`, `Standard_E8ads_V5`, `Standard_E16ads_V5`, `Standard_E20ads_V5`, `Standard_E32ads_V5`, `Standard_E48ads_V5`, `Standard_E64ads_V5`, `Standard_E96ads_V5`, `Standard_G1`, `Standard_G2`, `Standard_G3`, `Standard_G4`, `Standard_G5`, `Standard_F2s_V2`, `Standard_F4s_V2`, `Standard_F8s_V2`, `Standard_F16s_V2`, `Standard_F32s_V2`, `Standard_F64s_V2`, `Standard_F72s_V2`, `Standard_GS1`, `Standard_GS2`, `Standard_GS3`, `Standard_GS4`, `Standard_GS5` and `Standard_NC24`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "vmSize")]
    pub r#vm_size: Box<String>,
}
