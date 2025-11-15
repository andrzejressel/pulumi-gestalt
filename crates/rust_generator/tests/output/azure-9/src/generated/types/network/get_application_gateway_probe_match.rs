#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetApplicationGatewayProbeMatch {
    /// A snippet from the Response Body which must be present in the Response.
    #[builder(into)]
    #[serde(rename = "body")]
    pub r#body: String,
    /// Status code of the application gateway custom error.
    #[builder(into)]
    #[serde(rename = "statusCodes")]
    pub r#status_codes: Vec<String>,
}
