#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationGatewayRedirectConfiguration {
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Whether the path is included in the redirected URL.
    #[builder(into)]
    #[serde(rename = "includePath")]
    pub r#include_path: bool,
    /// Whether to include the query string in the redirected URL.
    #[builder(into)]
    #[serde(rename = "includeQueryString")]
    pub r#include_query_string: bool,
    /// The name of this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The type of redirect.
    #[builder(into)]
    #[serde(rename = "redirectType")]
    pub r#redirect_type: String,
    #[builder(into)]
    #[serde(rename = "targetListenerId")]
    pub r#target_listener_id: String,
    /// The name of the listener to redirect to.
    #[builder(into)]
    #[serde(rename = "targetListenerName")]
    pub r#target_listener_name: String,
    /// The URL to redirect the request to.
    #[builder(into)]
    #[serde(rename = "targetUrl")]
    pub r#target_url: String,
}
