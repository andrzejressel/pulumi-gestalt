#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppCacheConfig {
    /// Type of cache configuration to use for an Amplify app. Valid values: `AMPLIFY_MANAGED`, `AMPLIFY_MANAGED_NO_COOKIES`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
