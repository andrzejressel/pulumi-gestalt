#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IotHubDpsLinkedHub {
    /// The weight applied to the IoT Hub. Defaults to `1`.
    #[builder(into)]
    #[serde(rename = "allocationWeight")]
    pub r#allocation_weight: Option<i32>,
    /// Determines whether to apply allocation policies to the IoT Hub. Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "applyAllocationPolicy")]
    pub r#apply_allocation_policy: Option<bool>,
    /// The connection string to connect to the IoT Hub.
    #[builder(into)]
    #[serde(rename = "connectionString")]
    pub r#connection_string: String,
    /// The IoT Hub hostname.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Option<String>,
    /// The location of the IoT hub.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: String,
}
