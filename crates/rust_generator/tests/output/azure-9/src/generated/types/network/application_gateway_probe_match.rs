#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationGatewayProbeMatch {
    /// A snippet from the Response Body which must be present in the Response.
    #[builder(into)]
    #[serde(rename = "body")]
    pub r#body: Option<String>,
    /// A list of allowed status codes for this Health Probe.
    #[builder(into)]
    #[serde(rename = "statusCodes")]
    pub r#status_codes: Vec<String>,
}
