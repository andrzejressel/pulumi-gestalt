#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TlsInspectionConfigurationCertificate {
    /// ARN of the certificate.
    #[builder(into)]
    #[serde(rename = "certificateArn")]
    pub r#certificate_arn: String,
    /// Serial number of the certificate.
    #[builder(into)]
    #[serde(rename = "certificateSerial")]
    pub r#certificate_serial: String,
    /// Status of the certificate.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: String,
    /// Details about the certificate status, including information about certificate errors.
    #[builder(into)]
    #[serde(rename = "statusMessage")]
    pub r#status_message: String,
}
