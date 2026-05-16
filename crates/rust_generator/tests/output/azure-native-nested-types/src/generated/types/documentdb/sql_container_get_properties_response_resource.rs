#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SqlContainerGetPropertiesResponseResource {
    /// The configuration of the indexing policy. By default, the indexing is automatic for all document paths within the container
    #[builder(into)]
    #[serde(rename = "indexingPolicy")]
    pub r#indexing_policy: Option<Box<super::super::types::documentdb::IndexingPolicyResponse>>,
}
