#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserPoolSmsConfiguration {
    /// External ID used in IAM role trust relationships. For more information about using external IDs, see [How to Use an External ID When Granting Access to Your AWS Resources to a Third Party](http://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_create_for-user_externalid.html).
    #[builder(into)]
    #[serde(rename = "externalId")]
    pub r#external_id: Box<String>,
    /// ARN of the Amazon SNS caller. This is usually the IAM role that you've given Cognito permission to assume.
    #[builder(into)]
    #[serde(rename = "snsCallerArn")]
    pub r#sns_caller_arn: Box<String>,
    /// The AWS Region to use with Amazon SNS integration. You can choose the same Region as your user pool, or a supported Legacy Amazon SNS alternate Region. Amazon Cognito resources in the Asia Pacific (Seoul) AWS Region must use your Amazon SNS configuration in the Asia Pacific (Tokyo) Region. For more information, see [SMS message settings for Amazon Cognito user pools](https://docs.aws.amazon.com/cognito/latest/developerguide/user-pool-sms-settings.html).
    #[builder(into, default)]
    #[serde(rename = "snsRegion")]
    pub r#sns_region: Box<Option<String>>,
}
