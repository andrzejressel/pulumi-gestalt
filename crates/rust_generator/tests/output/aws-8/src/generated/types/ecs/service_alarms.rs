#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceAlarms {
    /// One or more CloudWatch alarm names.
    #[builder(into)]
    #[serde(rename = "alarmNames")]
    pub r#alarm_names: Vec<String>,
    /// Whether to use the CloudWatch alarm option in the service deployment process.
    #[builder(into)]
    #[serde(rename = "enable")]
    pub r#enable: bool,
    /// Whether to configure Amazon ECS to roll back the service if a service deployment fails. If rollback is used, when a service deployment fails, the service is rolled back to the last deployment that completed successfully.
    #[builder(into)]
    #[serde(rename = "rollback")]
    pub r#rollback: bool,
}
