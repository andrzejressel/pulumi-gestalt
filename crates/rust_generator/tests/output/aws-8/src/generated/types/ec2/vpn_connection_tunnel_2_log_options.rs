#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpnConnectionTunnel2LogOptions {
    /// Options for sending VPN tunnel logs to CloudWatch. See CloudWatch Log Options below for more details.
    #[builder(into)]
    #[serde(rename = "cloudwatchLogOptions")]
    pub r#cloudwatch_log_options: Option<Box<super::super::types::ec2::VpnConnectionTunnel2LogOptionsCloudwatchLogOptions>>,
}
