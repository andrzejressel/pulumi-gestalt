#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDeviceDeviceProperty {
    /// The Data Box Edge/Gateway device local capacity in MB.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: i32,
    /// Type of compute roles configured.
    #[builder(into)]
    #[serde(rename = "configuredRoleTypes")]
    pub r#configured_role_types: Vec<String>,
    /// The Data Box Edge/Gateway device culture.
    #[builder(into)]
    #[serde(rename = "culture")]
    pub r#culture: String,
    /// The device software version number of the device (e.g. 1.2.18105.6).
    #[builder(into)]
    #[serde(rename = "hcsVersion")]
    pub r#hcs_version: String,
    /// The Data Box Edge/Gateway device model.
    #[builder(into)]
    #[serde(rename = "model")]
    pub r#model: String,
    /// The number of nodes in the cluster.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: i32,
    /// The Serial Number of Data Box Edge/Gateway device.
    #[builder(into)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: String,
    /// The Data Box Edge/Gateway device software version.
    #[builder(into)]
    #[serde(rename = "softwareVersion")]
    pub r#software_version: String,
    /// The status of the Data Box Edge/Gateway device.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
    /// The Data Box Edge/Gateway device timezone.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: String,
    /// The type of the Data Box Edge/Gateway device.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
