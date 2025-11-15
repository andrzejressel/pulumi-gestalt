#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VolumeExportPolicyRule {
    /// Defines the access type for clients matching the `allowedClients` specification.
    /// Possible values are: `READ_ONLY`, `READ_WRITE`, `READ_NONE`.
    #[builder(into)]
    #[serde(rename = "accessType")]
    pub r#access_type: Option<String>,
    /// Defines the client ingress specification (allowed clients) as a comma separated list with IPv4 CIDRs or IPv4 host addresses.
    #[builder(into)]
    #[serde(rename = "allowedClients")]
    pub r#allowed_clients: Option<String>,
    /// If enabled, the root user (UID = 0) of the specified clients doesn't get mapped to nobody (UID = 65534). This is also known as no_root_squash.
    #[builder(into)]
    #[serde(rename = "hasRootAccess")]
    pub r#has_root_access: Option<String>,
    /// If enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'authentication' kerberos security mode.
    #[builder(into)]
    #[serde(rename = "kerberos5ReadOnly")]
    pub r#kerberos_5_read_only: Option<bool>,
    /// If enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'authentication' kerberos security mode. The 'kerberos5ReadOnly' value is ignored if this is enabled.
    #[builder(into)]
    #[serde(rename = "kerberos5ReadWrite")]
    pub r#kerberos_5_read_write: Option<bool>,
    /// If enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'integrity' kerberos security mode.
    #[builder(into)]
    #[serde(rename = "kerberos5iReadOnly")]
    pub r#kerberos_5_i_read_only: Option<bool>,
    /// If enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'integrity' kerberos security mode. The 'kerberos5iReadOnly' value is ignored if this is enabled.
    #[builder(into)]
    #[serde(rename = "kerberos5iReadWrite")]
    pub r#kerberos_5_i_read_write: Option<bool>,
    /// If enabled (true) the rule defines a read only access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'privacy' kerberos security mode.
    #[builder(into)]
    #[serde(rename = "kerberos5pReadOnly")]
    pub r#kerberos_5_p_read_only: Option<bool>,
    /// If enabled (true) the rule defines read and write access for clients matching the 'allowedClients' specification. It enables nfs clients to mount using 'privacy' kerberos security mode. The 'kerberos5pReadOnly' value is ignored if this is enabled.
    #[builder(into)]
    #[serde(rename = "kerberos5pReadWrite")]
    pub r#kerberos_5_p_read_write: Option<bool>,
    /// Enable to apply the export rule to NFSV3 clients.
    #[builder(into)]
    #[serde(rename = "nfsv3")]
    pub r#nfsv_3: Option<bool>,
    /// Enable to apply the export rule to NFSV4.1 clients.
    #[builder(into)]
    #[serde(rename = "nfsv4")]
    pub r#nfsv_4: Option<bool>,
}
