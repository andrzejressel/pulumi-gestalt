#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterAccessConfig {
    /// Values returned are `CONFIG_MAP`, `API` or `API_AND_CONFIG_MAP`
    #[builder(into)]
    #[serde(rename = "authenticationMode")]
    pub r#authentication_mode: String,
    /// Default to `true`.
    #[builder(into)]
    #[serde(rename = "bootstrapClusterCreatorAdminPermissions")]
    pub r#bootstrap_cluster_creator_admin_permissions: bool,
}
