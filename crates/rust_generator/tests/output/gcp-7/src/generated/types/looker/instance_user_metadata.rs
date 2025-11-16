#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceUserMetadata {
    /// Number of additional Developer Users to allocate to the Looker Instance.
    #[builder(into)]
    #[serde(rename = "additionalDeveloperUserCount")]
    pub r#additional_developer_user_count: Option<i32>,
    /// Number of additional Standard Users to allocate to the Looker Instance.
    #[builder(into)]
    #[serde(rename = "additionalStandardUserCount")]
    pub r#additional_standard_user_count: Option<i32>,
    /// Number of additional Viewer Users to allocate to the Looker Instance.
    #[builder(into)]
    #[serde(rename = "additionalViewerUserCount")]
    pub r#additional_viewer_user_count: Option<i32>,
}
