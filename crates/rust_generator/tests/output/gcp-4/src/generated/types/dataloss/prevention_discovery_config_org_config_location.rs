#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigOrgConfigLocation {
    /// The ID for the folder within an organization to scan
    #[builder(into)]
    #[serde(rename = "folderId")]
    pub r#folder_id: Option<String>,
    /// The ID of an organization to scan
    #[builder(into)]
    #[serde(rename = "organizationId")]
    pub r#organization_id: Option<String>,
}
