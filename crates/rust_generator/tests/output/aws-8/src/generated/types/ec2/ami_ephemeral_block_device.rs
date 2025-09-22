#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AmiEphemeralBlockDevice {
    /// Path at which the device is exposed to created instances.
    #[builder(into)]
    #[serde(rename = "deviceName")]
    pub r#device_name: String,
    /// Name for the ephemeral device, of the form "ephemeralN" where
    /// *N* is a volume number starting from zero.
    #[builder(into)]
    #[serde(rename = "virtualName")]
    pub r#virtual_name: String,
}
