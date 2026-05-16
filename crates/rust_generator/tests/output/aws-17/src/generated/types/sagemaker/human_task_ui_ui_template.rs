#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HumanTaskUiUiTemplate {
    /// The content of the Liquid template for the worker user interface.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// The SHA-256 digest of the contents of the template.
    #[builder(into)]
    #[serde(rename = "contentSha256")]
    pub r#content_sha_256: Option<String>,
    /// The URL for the user interface template.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}
