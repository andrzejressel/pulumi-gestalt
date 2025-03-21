#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VirtualNodeSpecBackend {
    /// Virtual service to use as a backend for a virtual node.
    #[builder(into)]
    #[serde(rename = "virtualService")]
    pub r#virtual_service: Box<super::super::types::appmesh::VirtualNodeSpecBackendVirtualService>,
}
