#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetDistributionConfigurationDistributionContainerDistributionConfigurationTargetRepository {
    /// Name of the container repository where the output container image is stored.
    #[builder(into)]
    #[serde(rename = "repositoryName")]
    pub r#repository_name: String,
    /// Service in which the image is registered.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: String,
}
