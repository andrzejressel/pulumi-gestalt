#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VpnConnectionTunnel2LogOptionsCloudwatchLogOptions {
    /// Enable or disable VPN tunnel logging feature. The default is `false`.
    #[builder(into)]
    #[serde(rename = "logEnabled")]
    pub r#log_enabled: Option<bool>,
    /// The Amazon Resource Name (ARN) of the CloudWatch log group to send logs to.
    #[builder(into)]
    #[serde(rename = "logGroupArn")]
    pub r#log_group_arn: Option<String>,
    /// Set log format. Default format is json. Possible values are: `json` and `text`. The default is `json`.
    #[builder(into)]
    #[serde(rename = "logOutputFormat")]
    pub r#log_output_format: Option<String>,
}
