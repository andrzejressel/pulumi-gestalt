#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeviceDeviceProperty {
    /// The Data Box Edge/Gateway device local capacity in MB.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: Option<i32>,
    /// Type of compute roles configured.
    #[builder(into)]
    #[serde(rename = "configuredRoleTypes")]
    pub r#configured_role_types: Option<Vec<String>>,
    /// The Data Box Edge/Gateway device culture.
    #[builder(into)]
    #[serde(rename = "culture")]
    pub r#culture: Option<String>,
    /// The device software version number of the device (e.g. 1.2.18105.6).
    #[builder(into)]
    #[serde(rename = "hcsVersion")]
    pub r#hcs_version: Option<String>,
    /// The Data Box Edge/Gateway device model.
    #[builder(into)]
    #[serde(rename = "model")]
    pub r#model: Option<String>,
    /// The number of nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Option<i32>,
    /// The Serial Number of Data Box Edge/Gateway device.
    #[builder(into)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Option<String>,
    /// The Data Box Edge/Gateway device software version.
    #[builder(into)]
    #[serde(rename = "softwareVersion")]
    pub r#software_version: Option<String>,
    /// The status of the Data Box Edge/Gateway device.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// The Data Box Edge/Gateway device timezone.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Option<String>,
    /// The type of the Data Box Edge/Gateway device.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
