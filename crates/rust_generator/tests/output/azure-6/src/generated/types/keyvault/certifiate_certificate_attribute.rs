#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertifiateCertificateAttribute {
    /// The create time of the Key Vault Certificate.
    #[builder(into)]
    #[serde(rename = "created")]
    pub r#created: Option<String>,
    /// whether the Key Vault Certificate is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The expires time of the Key Vault Certificate.
    #[builder(into)]
    #[serde(rename = "expires")]
    pub r#expires: Option<String>,
    /// The not before valid time of the Key Vault Certificate.
    #[builder(into)]
    #[serde(rename = "notBefore")]
    pub r#not_before: Option<String>,
    /// The deletion recovery level of the Key Vault Certificate.
    #[builder(into)]
    #[serde(rename = "recoveryLevel")]
    pub r#recovery_level: Option<String>,
    /// The recent update time of the Key Vault Certificate.
    #[builder(into)]
    #[serde(rename = "updated")]
    pub r#updated: Option<String>,
}
