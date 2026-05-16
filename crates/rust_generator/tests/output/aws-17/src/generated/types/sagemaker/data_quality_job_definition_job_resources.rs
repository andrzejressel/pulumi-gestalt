#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataQualityJobDefinitionJobResources {
    /// The configuration for the cluster resources used to run the processing job. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "clusterConfig")]
    pub r#cluster_config: Box<super::super::types::sagemaker::DataQualityJobDefinitionJobResourcesClusterConfig>,
}
