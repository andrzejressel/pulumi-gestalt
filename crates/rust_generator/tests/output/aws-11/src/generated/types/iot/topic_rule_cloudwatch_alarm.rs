#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TopicRuleCloudwatchAlarm {
    /// The CloudWatch alarm name.
    #[builder(into)]
    #[serde(rename = "alarmName")]
    pub r#alarm_name: String,
    /// The IAM role ARN that allows access to the CloudWatch alarm.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// The reason for the alarm change.
    #[builder(into)]
    #[serde(rename = "stateReason")]
    pub r#state_reason: String,
    /// The value of the alarm state. Acceptable values are: OK, ALARM, INSUFFICIENT_DATA.
    #[builder(into)]
    #[serde(rename = "stateValue")]
    pub r#state_value: String,
}
