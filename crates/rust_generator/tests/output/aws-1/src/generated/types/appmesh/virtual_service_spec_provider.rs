#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualServiceSpecProvider {
    /// Virtual node associated with a virtual service.
    #[builder(into)]
    #[serde(rename = "virtualNode")]
    pub r#virtual_node: Option<Box<super::super::types::appmesh::VirtualServiceSpecProviderVirtualNode>>,
    /// Virtual router associated with a virtual service.
    #[builder(into)]
    #[serde(rename = "virtualRouter")]
    pub r#virtual_router: Option<Box<super::super::types::appmesh::VirtualServiceSpecProviderVirtualRouter>>,
}
