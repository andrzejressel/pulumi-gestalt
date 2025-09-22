#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterClusterConfigEncryptionConfig {
    /// The Cloud KMS key name to use for PD disk encryption for
    /// all instances in the cluster.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "kmsKeyName")]
    pub r#kms_key_name: String,
}
