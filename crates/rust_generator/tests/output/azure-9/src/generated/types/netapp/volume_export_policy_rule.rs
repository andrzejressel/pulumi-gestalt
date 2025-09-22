#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VolumeExportPolicyRule {
    /// A list of allowed clients IPv4 addresses.
    #[builder(into)]
    #[serde(rename = "allowedClients")]
    pub r#allowed_clients: Vec<String>,
    /// Is Kerberos 5 read-only access permitted to this volume?
    #[builder(into)]
    #[serde(rename = "kerberos5ReadOnlyEnabled")]
    pub r#kerberos_5_read_only_enabled: Option<bool>,
    /// Is Kerberos 5 read/write permitted to this volume?
    #[builder(into)]
    #[serde(rename = "kerberos5ReadWriteEnabled")]
    pub r#kerberos_5_read_write_enabled: Option<bool>,
    /// Is Kerberos 5i read-only permitted to this volume?
    #[builder(into)]
    #[serde(rename = "kerberos5iReadOnlyEnabled")]
    pub r#kerberos_5_i_read_only_enabled: Option<bool>,
    /// Is Kerberos 5i read/write permitted to this volume?
    #[builder(into)]
    #[serde(rename = "kerberos5iReadWriteEnabled")]
    pub r#kerberos_5_i_read_write_enabled: Option<bool>,
    /// Is Kerberos 5p read-only permitted to this volume?
    #[builder(into)]
    #[serde(rename = "kerberos5pReadOnlyEnabled")]
    pub r#kerberos_5_p_read_only_enabled: Option<bool>,
    /// Is Kerberos 5p read/write permitted to this volume?
    #[builder(into)]
    #[serde(rename = "kerberos5pReadWriteEnabled")]
    pub r#kerberos_5_p_read_write_enabled: Option<bool>,
    /// A list of allowed protocols. Valid values include `CIFS`, `NFSv3`, or `NFSv4.1`. Only one value is supported at this time. This replaces the previous arguments: `cifs_enabled`, `nfsv3_enabled` and `nfsv4_enabled`.
    #[builder(into)]
    #[serde(rename = "protocolsEnabled")]
    pub r#protocols_enabled: Option<String>,
    /// Is root access permitted to this volume?
    #[builder(into)]
    #[serde(rename = "rootAccessEnabled")]
    pub r#root_access_enabled: Option<bool>,
    /// The index number of the rule.
    #[builder(into)]
    #[serde(rename = "ruleIndex")]
    pub r#rule_index: i32,
    /// Is the file system on unix read only?
    #[builder(into)]
    #[serde(rename = "unixReadOnly")]
    pub r#unix_read_only: Option<bool>,
    /// Is the file system on unix read and write?
    #[builder(into)]
    #[serde(rename = "unixReadWrite")]
    pub r#unix_read_write: Option<bool>,
}
