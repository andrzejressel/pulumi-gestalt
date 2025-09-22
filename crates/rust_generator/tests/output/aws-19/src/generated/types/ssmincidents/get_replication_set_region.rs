#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetReplicationSetRegion {
    /// The ARN of the AWS Key Management Service (AWS KMS) encryption key.
    #[builder(into)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: String,
    /// The name of the Region.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The current status of the Region.
    /// * Valid Values: `ACTIVE` | `CREATING` | `UPDATING` | `DELETING` | `FAILED`
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
    /// More information about the status of a Region.
    #[builder(into)]
    #[serde(rename = "statusMessage")]
    pub r#status_message: String,
}
