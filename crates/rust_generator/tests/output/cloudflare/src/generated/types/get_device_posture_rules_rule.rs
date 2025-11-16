#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDevicePostureRulesRule {
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Expire posture results after the specified amount of time. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    #[builder(into)]
    #[serde(rename = "expiration")]
    pub r#expiration: Option<String>,
    /// ID of the Device Posture Rule.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Name of the device posture rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Tells the client when to run the device posture check. Must be in the format `1h` or `30m`. Valid units are `h` and `m`.
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Option<String>,
    /// The device posture rule type. Available values: `serial_number`, `file`, `application`, `gateway`, `warp`, `domain_joined`, `os_version`, `disk_encryption`, `firewall`, `client_certificate`, `client_certificate_v2`, `workspace_one`, `unique_client_id`, `crowdstrike_s2s`, `sentinelone`, `kolide`, `tanium_s2s`, `intune`, `sentinelone_s2s`, `custom_s2s`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
