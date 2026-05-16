#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HttpRouteRuleActionRequestMirrorPolicy {
    /// The destination the requests will be mirrored to.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Option<Box<super::super::types::networkservices::HttpRouteRuleActionRequestMirrorPolicyDestination>>,
}
