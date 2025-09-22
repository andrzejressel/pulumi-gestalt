#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GatewayRouteSpecHttpRouteActionTargetVirtualService {
    /// Name of the virtual service that traffic is routed to. Must be between 1 and 255 characters in length.
    #[builder(into)]
    #[serde(rename = "virtualServiceName")]
    pub r#virtual_service_name: String,
}
