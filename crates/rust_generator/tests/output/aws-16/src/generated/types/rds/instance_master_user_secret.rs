#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceMasterUserSecret {
    /// The ARN for the KMS encryption key. If creating an
    /// encrypted replica, set this to the destination KMS ARN.
    #[builder(into)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Option<String>,
    /// The Amazon Resource Name (ARN) of the secret.
    #[builder(into)]
    #[serde(rename = "secretArn")]
    pub r#secret_arn: Option<String>,
    /// The status of the secret. Valid Values: `creating` | `active` | `rotating` | `impaired`.
    #[builder(into)]
    #[serde(rename = "secretStatus")]
    pub r#secret_status: Option<String>,
}
