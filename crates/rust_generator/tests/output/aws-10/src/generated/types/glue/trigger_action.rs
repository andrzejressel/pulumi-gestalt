#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TriggerAction {
    /// Arguments to be passed to the job. You can specify arguments here that your own job-execution script consumes, as well as arguments that AWS Glue itself consumes.
    #[builder(into)]
    #[serde(rename = "arguments")]
    pub r#arguments: Option<std::collections::HashMap<String, String>>,
    /// The name of the crawler to be executed. Conflicts with `job_name`.
    #[builder(into)]
    #[serde(rename = "crawlerName")]
    pub r#crawler_name: Option<String>,
    /// The name of a job to be executed. Conflicts with `crawler_name`.
    #[builder(into)]
    #[serde(rename = "jobName")]
    pub r#job_name: Option<String>,
    /// Specifies configuration properties of a job run notification. See Notification Property details below.
    #[builder(into)]
    #[serde(rename = "notificationProperty")]
    pub r#notification_property: Option<Box<super::super::types::glue::TriggerActionNotificationProperty>>,
    /// The name of the Security Configuration structure to be used with this action.
    #[builder(into)]
    #[serde(rename = "securityConfiguration")]
    pub r#security_configuration: Option<String>,
    /// The job run timeout in minutes. It overrides the timeout value of the job.
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<i32>,
}
