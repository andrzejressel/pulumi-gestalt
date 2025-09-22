#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GatewayRouteSpecHttp2RouteActionRewriteHostname {
    /// Default target host name to write to. Valid values: `ENABLED`, `DISABLED`.
    #[builder(into)]
    #[serde(rename = "defaultTargetHostname")]
    pub r#default_target_hostname: String,
}
