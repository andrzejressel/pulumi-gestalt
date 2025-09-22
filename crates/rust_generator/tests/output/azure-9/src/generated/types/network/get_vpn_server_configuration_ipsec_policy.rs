#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetVpnServerConfigurationIpsecPolicy {
    /// The DH Group, used in IKE Phase 1.
    #[builder(into)]
    #[serde(rename = "dhGroup")]
    pub r#dh_group: String,
    /// The IKE encryption algorithm, used for IKE Phase 2.
    #[builder(into)]
    #[serde(rename = "ikeEncryption")]
    pub r#ike_encryption: String,
    /// The IKE encryption integrity algorithm, used for IKE Phase 2.
    #[builder(into)]
    #[serde(rename = "ikeIntegrity")]
    pub r#ike_integrity: String,
    /// The IPSec encryption algorithm, used for IKE phase 1.
    #[builder(into)]
    #[serde(rename = "ipsecEncryption")]
    pub r#ipsec_encryption: String,
    /// The IPSec integrity algorithm, used for IKE phase 1.
    #[builder(into)]
    #[serde(rename = "ipsecIntegrity")]
    pub r#ipsec_integrity: String,
    /// The Pfs Group, used in IKE Phase 2.
    #[builder(into)]
    #[serde(rename = "pfsGroup")]
    pub r#pfs_group: String,
    /// The IPSec Security Association payload size in KB for a Site-to-Site VPN tunnel.
    #[builder(into)]
    #[serde(rename = "saDataSizeKilobytes")]
    pub r#sa_data_size_kilobytes: i32,
    /// The IPSec Security Association lifetime in seconds for a Site-to-Site VPN tunnel.
    #[builder(into)]
    #[serde(rename = "saLifetimeSeconds")]
    pub r#sa_lifetime_seconds: i32,
}
