#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AppTemplateHttpScaleRule {
    /// Zero or more `authentication` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "authentications")]
    pub r#authentications: Option<Vec<super::super::types::containerapp::AppTemplateHttpScaleRuleAuthentication>>,
    /// The number of concurrent requests to trigger scaling.
    #[builder(into)]
    #[serde(rename = "concurrentRequests")]
    pub r#concurrent_requests: String,
    /// The name of the Scaling Rule
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
