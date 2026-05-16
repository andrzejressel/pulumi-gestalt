#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationGatewayHttpListenerCustomErrorConfiguration {
    /// Error page URL of the application gateway custom error.
    #[builder(into)]
    #[serde(rename = "customErrorPageUrl")]
    pub r#custom_error_page_url: String,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Status code of the application gateway custom error.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: String,
}
