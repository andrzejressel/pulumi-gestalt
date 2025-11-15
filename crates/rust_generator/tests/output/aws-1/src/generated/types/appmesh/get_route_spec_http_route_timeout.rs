#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRouteSpecHttpRouteTimeout {
    #[builder(into)]
    #[serde(rename = "idles")]
    pub r#idles: Vec<super::super::types::appmesh::GetRouteSpecHttpRouteTimeoutIdle>,
    #[builder(into)]
    #[serde(rename = "perRequests")]
    pub r#per_requests: Vec<super::super::types::appmesh::GetRouteSpecHttpRouteTimeoutPerRequest>,
}
