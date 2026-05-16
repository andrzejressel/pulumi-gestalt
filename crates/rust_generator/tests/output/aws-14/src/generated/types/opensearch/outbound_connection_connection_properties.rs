#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OutboundConnectionConnectionProperties {
    /// Configuration block for cross cluster search.
    #[builder(into)]
    #[serde(rename = "crossClusterSearch")]
    pub r#cross_cluster_search: Option<Box<super::super::types::opensearch::OutboundConnectionConnectionPropertiesCrossClusterSearch>>,
    /// The endpoint of the remote domain, is only set when `connection_mode` is `VPC_ENDPOINT` and `accept_connection` is `TRUE`.
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: Option<String>,
}
