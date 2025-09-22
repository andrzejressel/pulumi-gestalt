#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterUserManagedKeysConfig {
    /// The Certificate Authority Service caPool to use for the aggreation CA in this cluster.
    #[builder(into)]
    #[serde(rename = "aggregationCa")]
    pub r#aggregation_ca: String,
    /// The Certificate Authority Service caPool to use for the cluster CA in this cluster.
    #[builder(into)]
    #[serde(rename = "clusterCa")]
    pub r#cluster_ca: String,
    /// The Cloud KMS cryptoKey to use for Confidential Hyperdisk on the control plane nodes.
    #[builder(into)]
    #[serde(rename = "controlPlaneDiskEncryptionKey")]
    pub r#control_plane_disk_encryption_key: String,
    /// The Certificate Authority Service caPool to use for the etcd API CA in this cluster.
    #[builder(into)]
    #[serde(rename = "etcdApiCa")]
    pub r#etcd_api_ca: String,
    /// The Certificate Authority Service caPool to use for the etcd peer CA in this cluster.
    #[builder(into)]
    #[serde(rename = "etcdPeerCa")]
    pub r#etcd_peer_ca: String,
    /// Resource path of the Cloud KMS cryptoKey to use for encryption of internal etcd backups.
    #[builder(into)]
    #[serde(rename = "gkeopsEtcdBackupEncryptionKey")]
    pub r#gkeops_etcd_backup_encryption_key: String,
    /// The Cloud KMS cryptoKeyVersions to use for signing service account JWTs issued by this cluster.
    #[builder(into)]
    #[serde(rename = "serviceAccountSigningKeys")]
    pub r#service_account_signing_keys: Vec<String>,
    /// The Cloud KMS cryptoKeyVersions to use for verifying service account JWTs issued by this cluster.
    #[builder(into)]
    #[serde(rename = "serviceAccountVerificationKeys")]
    pub r#service_account_verification_keys: Vec<String>,
}
