#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetConfigurationProtectedFile {
    /// The base-64 encoded contents of this configuration file.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: String,
    /// The path of this configuration file.
    #[builder(into)]
    #[serde(rename = "virtualPath")]
    pub r#virtual_path: String,
}
