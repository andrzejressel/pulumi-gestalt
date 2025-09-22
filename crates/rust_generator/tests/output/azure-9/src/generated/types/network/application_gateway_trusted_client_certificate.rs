#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApplicationGatewayTrustedClientCertificate {
    /// The base-64 encoded certificate.
    #[builder(into)]
    #[serde(rename = "data")]
    pub r#data: String,
    /// The ID of the Rewrite Rule Set
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The name of the Trusted Client Certificate that is unique within this Application Gateway.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
