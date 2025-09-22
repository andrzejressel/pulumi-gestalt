#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinuxWebAppSlotSiteConfigAutoHealSettingTrigger {
    /// A `requests` block as defined above.
    #[builder(into)]
    #[serde(rename = "requests")]
    pub r#requests: Option<Box<super::super::types::appservice::LinuxWebAppSlotSiteConfigAutoHealSettingTriggerRequests>>,
    /// A `slow_request` block as defined above.
    #[builder(into)]
    #[serde(rename = "slowRequest")]
    pub r#slow_request: Option<Box<super::super::types::appservice::LinuxWebAppSlotSiteConfigAutoHealSettingTriggerSlowRequest>>,
    /// One or more `slow_request_with_path` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "slowRequestWithPaths")]
    pub r#slow_request_with_paths: Option<Vec<super::super::types::appservice::LinuxWebAppSlotSiteConfigAutoHealSettingTriggerSlowRequestWithPath>>,
    /// One or more `status_code` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "statusCodes")]
    pub r#status_codes: Option<Vec<super::super::types::appservice::LinuxWebAppSlotSiteConfigAutoHealSettingTriggerStatusCode>>,
}
