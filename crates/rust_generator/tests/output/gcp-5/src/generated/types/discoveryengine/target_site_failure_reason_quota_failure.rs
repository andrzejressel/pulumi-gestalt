#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TargetSiteFailureReasonQuotaFailure {
    /// This number is an estimation on how much total quota this project
    /// needs to successfully complete indexing.
    #[builder(into)]
    #[serde(rename = "totalRequiredQuota")]
    pub r#total_required_quota: Option<i32>,
}
