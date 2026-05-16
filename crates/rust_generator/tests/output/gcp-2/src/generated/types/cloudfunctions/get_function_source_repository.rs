#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetFunctionSourceRepository {
    /// The URL pointing to the hosted repository where the function was defined at the time of deployment.
    #[builder(into)]
    #[serde(rename = "deployedUrl")]
    pub r#deployed_url: String,
    /// The URL pointing to the hosted repository where the function is defined.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
}
