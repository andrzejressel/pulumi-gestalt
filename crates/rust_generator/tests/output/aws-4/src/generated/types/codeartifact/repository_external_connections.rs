#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RepositoryExternalConnections {
    /// The name of the external connection associated with a repository.
    #[builder(into)]
    #[serde(rename = "externalConnectionName")]
    pub r#external_connection_name: String,
    #[builder(into)]
    #[serde(rename = "packageFormat")]
    pub r#package_format: Option<String>,
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Option<String>,
}
