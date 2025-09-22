#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetGatewayRouteSpecHttpRouteMatchHostname {
    #[builder(into)]
    #[serde(rename = "exact")]
    pub r#exact: String,
    #[builder(into)]
    #[serde(rename = "suffix")]
    pub r#suffix: String,
}
