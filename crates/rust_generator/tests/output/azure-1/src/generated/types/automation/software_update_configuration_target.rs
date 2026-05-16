#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SoftwareUpdateConfigurationTarget {
    /// One or more `azure_query` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "azureQueries")]
    pub r#azure_queries: Option<Vec<super::super::types::automation::SoftwareUpdateConfigurationTargetAzureQuery>>,
    /// One or more `non_azure_query` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "nonAzureQueries")]
    pub r#non_azure_queries: Option<Vec<super::super::types::automation::SoftwareUpdateConfigurationTargetNonAzureQuery>>,
}
