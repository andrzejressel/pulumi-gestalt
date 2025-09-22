#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GatewayRouteSpecHttpRouteActionRewritePrefix {
    /// Default prefix used to replace the incoming route prefix when rewritten. Valid values: `ENABLED`, `DISABLED`.
    #[builder(into)]
    #[serde(rename = "defaultPrefix")]
    pub r#default_prefix: Option<String>,
    /// Value used to replace the incoming route prefix when rewritten.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
