#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KeystoresAliasesKeyCertFileCertsInfoCertInfo {
    /// (Output)
    /// X.509 basic constraints extension.
    #[builder(into)]
    #[serde(rename = "basicConstraints")]
    pub r#basic_constraints: Option<String>,
    /// (Output)
    /// X.509 notAfter validity period in milliseconds since epoch.
    #[builder(into)]
    #[serde(rename = "expiryDate")]
    pub r#expiry_date: Option<String>,
    /// (Output)
    /// Flag that specifies whether the certificate is valid.
    /// Flag is set to Yes if the certificate is valid, No if expired, or Not yet if not yet valid.
    #[builder(into)]
    #[serde(rename = "isValid")]
    pub r#is_valid: Option<String>,
    /// (Output)
    /// X.509 issuer.
    #[builder(into)]
    #[serde(rename = "issuer")]
    pub r#issuer: Option<String>,
    /// (Output)
    /// Public key component of the X.509 subject public key info.
    #[builder(into)]
    #[serde(rename = "publicKey")]
    pub r#public_key: Option<String>,
    /// (Output)
    /// X.509 serial number.
    #[builder(into)]
    #[serde(rename = "serialNumber")]
    pub r#serial_number: Option<String>,
    /// (Output)
    /// X.509 signatureAlgorithm.
    #[builder(into)]
    #[serde(rename = "sigAlgName")]
    pub r#sig_alg_name: Option<String>,
    /// (Output)
    /// X.509 subject.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Option<String>,
    /// (Output)
    /// X.509 subject alternative names (SANs) extension.
    #[builder(into)]
    #[serde(rename = "subjectAlternativeNames")]
    pub r#subject_alternative_names: Option<Vec<String>>,
    /// (Output)
    /// X.509 notBefore validity period in milliseconds since epoch.
    #[builder(into)]
    #[serde(rename = "validFrom")]
    pub r#valid_from: Option<String>,
    /// (Output)
    /// X.509 version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<i32>,
}
