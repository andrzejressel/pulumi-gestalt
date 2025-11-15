#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PlanRuleCopyActionLifecycle {
    /// Specifies the number of days after creation that a recovery point is moved to cold storage.
    #[builder(into)]
    #[serde(rename = "coldStorageAfter")]
    pub r#cold_storage_after: Option<i32>,
    /// Specifies the number of days after creation that a recovery point is deleted. Must be 90 days greater than `cold_storage_after`.
    #[builder(into)]
    #[serde(rename = "deleteAfter")]
    pub r#delete_after: Option<i32>,
    /// This setting will instruct your backup plan to transition supported resources to archive (cold) storage tier in accordance with your lifecycle settings.
    #[builder(into)]
    #[serde(rename = "optInToArchiveForSupportedResources")]
    pub r#opt_in_to_archive_for_supported_resources: Option<bool>,
}
