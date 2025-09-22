#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionJobTriggerInspectJobAction {
    /// Create a de-identified copy of the requested table or files.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "deidentify")]
    pub r#deidentify: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionDeidentify>>,
    /// Sends an email when the job completes. The email goes to IAM project owners and technical Essential Contacts.
    #[builder(into)]
    #[serde(rename = "jobNotificationEmails")]
    pub r#job_notification_emails: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionJobNotificationEmails>>,
    /// Publish a message into a given Pub/Sub topic when the job completes.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pubSub")]
    pub r#pub_sub: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionPubSub>>,
    /// Publish findings of a DlpJob to Data Catalog.
    #[builder(into)]
    #[serde(rename = "publishFindingsToCloudDataCatalog")]
    pub r#publish_findings_to_cloud_data_catalog: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionPublishFindingsToCloudDataCatalog>>,
    /// Publish the result summary of a DlpJob to the Cloud Security Command Center.
    #[builder(into)]
    #[serde(rename = "publishSummaryToCscc")]
    pub r#publish_summary_to_cscc: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionPublishSummaryToCscc>>,
    /// Enable Stackdriver metric dlp.googleapis.com/findingCount.
    #[builder(into)]
    #[serde(rename = "publishToStackdriver")]
    pub r#publish_to_stackdriver: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionPublishToStackdriver>>,
    /// If set, the detailed findings will be persisted to the specified OutputStorageConfig. Only a single instance of this action can be specified. Compatible with: Inspect, Risk
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "saveFindings")]
    pub r#save_findings: Option<Box<super::super::types::dataloss::PreventionJobTriggerInspectJobActionSaveFindings>>,
}
