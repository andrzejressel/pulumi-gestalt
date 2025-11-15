#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GatewayRouteSpecHttpRouteActionRewrite {
    /// Host name to rewrite.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Option<Box<super::super::types::appmesh::GatewayRouteSpecHttpRouteActionRewriteHostname>>,
    /// Exact path to rewrite.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<Box<super::super::types::appmesh::GatewayRouteSpecHttpRouteActionRewritePath>>,
    /// Specified beginning characters to rewrite.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: Option<Box<super::super::types::appmesh::GatewayRouteSpecHttpRouteActionRewritePrefix>>,
}
