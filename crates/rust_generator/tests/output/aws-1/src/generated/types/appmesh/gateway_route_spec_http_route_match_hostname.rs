#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GatewayRouteSpecHttpRouteMatchHostname {
    /// Exact host name to match on.
    #[builder(into)]
    #[serde(rename = "exact")]
    pub r#exact: Option<String>,
    /// Specified ending characters of the host name to match on.
    #[builder(into)]
    #[serde(rename = "suffix")]
    pub r#suffix: Option<String>,
}
