#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProjectSecondarySourceBuildStatusConfig {
    /// Specifies the context of the build status CodeBuild sends to the source provider. The usage of this parameter depends on the source provider.
    #[builder(into)]
    #[serde(rename = "context")]
    pub r#context: Option<String>,
    /// Specifies the target url of the build status CodeBuild sends to the source provider. The usage of this parameter depends on the source provider.
    #[builder(into)]
    #[serde(rename = "targetUrl")]
    pub r#target_url: Option<String>,
}
