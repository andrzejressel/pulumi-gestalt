#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EventDestinationKinesisDestination {
    /// The ARN of the role that has permissions to access the Kinesis Stream
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// The ARN of the Kinesis Stream
    #[builder(into)]
    #[serde(rename = "streamArn")]
    pub r#stream_arn: Box<String>,
}
