#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeviceDevice {
    /// A description for the device.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The name of the device.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: String,
    /// Amazon Web Services Internet of Things (IoT) object name.
    #[builder(into)]
    #[serde(rename = "iotThingName")]
    pub r#iot_thing_name: Option<String>,
}
