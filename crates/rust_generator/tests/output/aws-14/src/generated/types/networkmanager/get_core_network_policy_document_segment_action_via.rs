#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetCoreNetworkPolicyDocumentSegmentActionVia {
    /// A list of strings. The network function group to use for the service insertion action.
    #[builder(into)]
    #[serde(rename = "networkFunctionGroups")]
    pub r#network_function_groups: Option<Vec<String>>,
    /// Any edge overrides and the preferred edge to use.
    #[builder(into)]
    #[serde(rename = "withEdgeOverrides")]
    pub r#with_edge_overrides: Option<Vec<super::super::types::networkmanager::GetCoreNetworkPolicyDocumentSegmentActionViaWithEdgeOverride>>,
}
