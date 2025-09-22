#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HttpRouteRuleActionRequestMirrorPolicy {
    /// The destination the requests will be mirrored to.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Option<Box<super::super::types::networkservices::HttpRouteRuleActionRequestMirrorPolicyDestination>>,
}
