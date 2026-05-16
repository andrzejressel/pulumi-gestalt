#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualRouterSpec {
    /// Listeners that the virtual router is expected to receive inbound traffic from.
    /// Currently only one listener is supported per virtual router.
    #[builder(into)]
    #[serde(rename = "listeners")]
    pub r#listeners: Option<Vec<super::super::types::appmesh::VirtualRouterSpecListener>>,
}
