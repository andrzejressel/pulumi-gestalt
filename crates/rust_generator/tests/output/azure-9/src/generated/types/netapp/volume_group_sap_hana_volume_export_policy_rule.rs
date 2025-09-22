#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VolumeGroupSapHanaVolumeExportPolicyRule {
    /// A comma-sperated list of allowed client IPv4 addresses.
    #[builder(into)]
    #[serde(rename = "allowedClients")]
    pub r#allowed_clients: String,
    /// Enables NFSv3. Please note that this cannot be enabled if volume has NFSv4.1 as its protocol.
    #[builder(into)]
    #[serde(rename = "nfsv3Enabled")]
    pub r#nfsv_3_enabled: bool,
    /// Enables NFSv4.1. Please note that this cannot be enabled if volume has NFSv3 as its protocol.
    #[builder(into)]
    #[serde(rename = "nfsv41Enabled")]
    pub r#nfsv_41_enabled: bool,
    /// Is root access permitted to this volume? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "rootAccessEnabled")]
    pub r#root_access_enabled: Option<bool>,
    /// The index number of the rule, must start at 1 and maximum 5.
    #[builder(into)]
    #[serde(rename = "ruleIndex")]
    pub r#rule_index: i32,
    /// Is the file system on unix read only? Defaults to `false.
    #[builder(into)]
    #[serde(rename = "unixReadOnly")]
    pub r#unix_read_only: Option<bool>,
    /// Is the file system on unix read and write? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "unixReadWrite")]
    pub r#unix_read_write: Option<bool>,
}
