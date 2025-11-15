#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GrpcRouteRule {
    /// Required. A detailed rule defining how to route traffic.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Option<Box<super::super::types::networkservices::GrpcRouteRuleAction>>,
    /// Matches define conditions used for matching the rule against incoming gRPC requests.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "matches")]
    pub r#matches: Option<Vec<super::super::types::networkservices::GrpcRouteRuleMatch>>,
}
