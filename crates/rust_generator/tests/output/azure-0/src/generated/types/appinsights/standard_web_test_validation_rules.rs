#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StandardWebTestValidationRules {
    /// A `content` block as defined above.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Option<Box<super::super::types::appinsights::StandardWebTestValidationRulesContent>>,
    /// The expected status code of the response. Default is '200', '0' means 'response code < 400'
    #[builder(into)]
    #[serde(rename = "expectedStatusCode")]
    pub r#expected_status_code: Option<i32>,
    /// The number of days of SSL certificate validity remaining for the checked endpoint. If the certificate has a shorter remaining lifetime left, the test will fail. This number should be between 1 and 365.
    #[builder(into)]
    #[serde(rename = "sslCertRemainingLifetime")]
    pub r#ssl_cert_remaining_lifetime: Option<i32>,
    /// Should the SSL check be enabled?
    #[builder(into)]
    #[serde(rename = "sslCheckEnabled")]
    pub r#ssl_check_enabled: Option<bool>,
}
