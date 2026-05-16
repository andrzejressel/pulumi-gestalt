#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserAuthenticationMode {
    /// Number of passwords belonging to the user if `type` is set to `password`.
    #[builder(into)]
    #[serde(rename = "passwordCount")]
    pub r#password_count: i32,
    /// Type of authentication configured.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
