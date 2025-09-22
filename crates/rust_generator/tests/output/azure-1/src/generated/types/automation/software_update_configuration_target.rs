#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
