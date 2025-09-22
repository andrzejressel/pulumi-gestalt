#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceServerCaCert {
    /// (Output)
    /// The certificate data in PEM format.
    #[builder(into)]
    #[serde(rename = "cert")]
    pub r#cert: Option<String>,
    /// (Output)
    /// The time when the certificate was created.
    #[builder(into)]
    #[serde(rename = "createTime")]
    pub r#create_time: Option<String>,
    /// (Output)
    /// The time when the certificate expires.
    #[builder(into)]
    #[serde(rename = "expireTime")]
    pub r#expire_time: Option<String>,
    /// (Output)
    /// Serial number, as extracted from the certificate.
    #[builder(into)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Option<String>,
    /// (Output)
    /// Sha1 Fingerprint of the certificate.
    #[builder(into)]
    #[serde(rename = "sha1Fingerprint")]
    pub r#sha_1_fingerprint: Option<String>,
}
