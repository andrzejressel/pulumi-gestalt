#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClassificationJobUserPausedDetail {
    #[builder(into)]
    #[serde(rename = "jobExpiresAt")]
    pub r#job_expires_at: Option<String>,
    #[builder(into)]
    #[serde(rename = "jobImminentExpirationHealthEventArn")]
    pub r#job_imminent_expiration_health_event_arn: Option<String>,
    #[builder(into)]
    #[serde(rename = "jobPausedAt")]
    pub r#job_paused_at: Option<String>,
}
