#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ThreeTierVirtualInstanceThreeTierConfiguration {
    #[builder(into)]
    #[serde(rename = "appResourceGroupName")]
    pub r#app_resource_group_name: String,
    /// An `application_server_configuration` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "applicationServerConfiguration")]
    pub r#application_server_configuration: Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationApplicationServerConfiguration>,
    /// A `central_server_configuration` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "centralServerConfiguration")]
    pub r#central_server_configuration: Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationCentralServerConfiguration>,
    /// A `database_server_configuration` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "databaseServerConfiguration")]
    pub r#database_server_configuration: Box<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationDatabaseServerConfiguration>,
    /// The high availability type for the three tier configuration. Possible values are `AvailabilitySet` and `AvailabilityZone`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "highAvailabilityType")]
    pub r#high_availability_type: Option<String>,
    /// A `resource_names` block as defined below. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "resourceNames")]
    pub r#resource_names: Box<Option<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationResourceNames>>,
    /// Specifies whether a secondary IP address should be added to the network interface on all VMs of the SAP system being deployed. Defaults to `false`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "secondaryIpEnabled")]
    pub r#secondary_ip_enabled: Option<bool>,
    /// A `transport_create_and_mount` block as defined below. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** The file share configuration uses `skip` by default when `transport_create_and_mount` isn't set.
    /// 
    /// > **Note:** Due to [a bug in the Azure API](https://github.com/Azure/azure-rest-api-specs/issues/25209) where the Storage File Share Id is not defined correctly, it is not currently possible to support using Transport Mount.
    #[builder(into)]
    #[serde(rename = "transportCreateAndMount")]
    pub r#transport_create_and_mount: Box<Option<super::super::types::workloadssap::ThreeTierVirtualInstanceThreeTierConfigurationTransportCreateAndMount>>,
}
