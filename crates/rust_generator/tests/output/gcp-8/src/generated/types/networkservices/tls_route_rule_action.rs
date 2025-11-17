#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TlsRouteRuleAction {
    /// The destination to which traffic should be forwarded.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "destinations")]
    pub r#destinations: Option<Vec<super::super::types::networkservices::TlsRouteRuleActionDestination>>,
}
