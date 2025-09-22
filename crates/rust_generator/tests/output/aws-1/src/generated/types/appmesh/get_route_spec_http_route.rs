#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRouteSpecHttpRoute {
    #[builder(into)]
    #[serde(rename = "actions")]
    pub r#actions: Vec<super::super::types::appmesh::GetRouteSpecHttpRouteAction>,
    #[builder(into)]
    #[serde(rename = "matches")]
    pub r#matches: Vec<super::super::types::appmesh::GetRouteSpecHttpRouteMatch>,
    #[builder(into)]
    #[serde(rename = "retryPolicies")]
    pub r#retry_policies: Vec<super::super::types::appmesh::GetRouteSpecHttpRouteRetryPolicy>,
    #[builder(into)]
    #[serde(rename = "timeouts")]
    pub r#timeouts: Vec<super::super::types::appmesh::GetRouteSpecHttpRouteTimeout>,
}
