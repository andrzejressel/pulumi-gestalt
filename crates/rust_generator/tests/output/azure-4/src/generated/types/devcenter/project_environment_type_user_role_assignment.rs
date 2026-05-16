#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProjectEnvironmentTypeUserRoleAssignment {
    /// A list of roles to assign to the `user_id`.
    #[builder(into)]
    #[serde(rename = "roles")]
    pub r#roles: Vec<String>,
    /// The user object ID that is assigned roles.
    #[builder(into)]
    #[serde(rename = "userId")]
    pub r#user_id: String,
}
