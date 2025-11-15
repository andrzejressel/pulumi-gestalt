#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetMeshSpec {
    #[builder(into)]
    #[serde(rename = "egressFilters")]
    pub r#egress_filters: Vec<super::super::types::appmesh::GetMeshSpecEgressFilter>,
    #[builder(into)]
    #[serde(rename = "serviceDiscoveries")]
    pub r#service_discoveries: Vec<super::super::types::appmesh::GetMeshSpecServiceDiscovery>,
}
