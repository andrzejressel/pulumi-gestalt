#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpotInstanceRequestMaintenanceOptions {
    /// Automatic recovery behavior of the Instance. Can be `"default"` or `"disabled"`. See [Recover your instance](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-recover.html) for more details.
    #[builder(into)]
    #[serde(rename = "autoRecovery")]
    pub r#auto_recovery: Option<String>,
}
