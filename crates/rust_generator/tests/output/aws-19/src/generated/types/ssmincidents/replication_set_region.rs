#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReplicationSetRegion {
    /// The Amazon Resource name (ARN) of the customer managed key. If omitted, AWS manages the AWS KMS keys for you, using an AWS owned key, as indicated by a default value of `DefaultKey`.
    /// 
    /// The following arguments are optional:
    #[builder(into)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Option<String>,
    /// The name of the Region, such as `ap-southeast-2`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The current status of the Region.
    /// * Valid Values: `ACTIVE` | `CREATING` | `UPDATING` | `DELETING` | `FAILED`
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
    /// More information about the status of a Region.
    #[builder(into)]
    #[serde(rename = "statusMessage")]
    pub r#status_message: Option<String>,
}
