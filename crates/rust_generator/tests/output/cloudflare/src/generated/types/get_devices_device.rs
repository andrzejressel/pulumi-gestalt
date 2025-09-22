#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDevicesDevice {
    /// When the device was created.
    #[builder(into)]
    #[serde(rename = "created")]
    pub r#created: Option<String>,
    /// Whether the device has been deleted.
    #[builder(into)]
    #[serde(rename = "deleted")]
    pub r#deleted: Option<bool>,
    /// The type of the device.
    #[builder(into)]
    #[serde(rename = "deviceType")]
    pub r#device_type: Option<String>,
    /// Device ID.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// IPv4 or IPv6 address.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Option<String>,
    /// The device's public key.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// When the device was last seen.
    #[builder(into)]
    #[serde(rename = "lastSeen")]
    pub r#last_seen: Option<String>,
    /// The device's MAC address.
    #[builder(into)]
    #[serde(rename = "macAddress")]
    pub r#mac_address: Option<String>,
    /// The device manufacturer's name.
    #[builder(into)]
    #[serde(rename = "manufacturer")]
    pub r#manufacturer: Option<String>,
    /// The device model name.
    #[builder(into)]
    #[serde(rename = "model")]
    pub r#model: Option<String>,
    /// The device name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// The Linux distribution name.
    #[builder(into)]
    #[serde(rename = "osDistroName")]
    pub r#os_distro_name: Option<String>,
    /// The Linux distribution revision.
    #[builder(into)]
    #[serde(rename = "osDistroRevision")]
    pub r#os_distro_revision: Option<String>,
    /// The operating system version.
    #[builder(into)]
    #[serde(rename = "osVersion")]
    pub r#os_version: Option<String>,
    /// Extra version value following the operating system version.
    #[builder(into)]
    #[serde(rename = "osVersionExtra")]
    pub r#os_version_extra: Option<String>,
    /// When the device was revoked.
    #[builder(into)]
    #[serde(rename = "revokedAt")]
    pub r#revoked_at: Option<String>,
    /// The device's serial number.
    #[builder(into)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Option<String>,
    /// When the device was updated.
    #[builder(into)]
    #[serde(rename = "updated")]
    pub r#updated: Option<String>,
    /// User's email.
    #[builder(into)]
    #[serde(rename = "userEmail")]
    pub r#user_email: Option<String>,
    /// User's ID.
    #[builder(into)]
    #[serde(rename = "userId")]
    pub r#user_id: Option<String>,
    /// User's Name.
    #[builder(into)]
    #[serde(rename = "userName")]
    pub r#user_name: Option<String>,
    /// The WARP client version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
