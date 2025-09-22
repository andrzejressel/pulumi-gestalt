#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateConfigX509Config {
    /// (Output)
    /// Describes custom X.509 extensions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "additionalExtensions")]
    pub r#additional_extensions: Option<Vec<super::super::types::certificateauthority::CertificateConfigX509ConfigAdditionalExtension>>,
    /// (Output)
    /// Describes Online Certificate Status Protocol (OCSP) endpoint addresses that appear in the
    /// "Authority Information Access" extension in the certificate.
    #[builder(into)]
    #[serde(rename = "aiaOcspServers")]
    pub r#aia_ocsp_servers: Option<Vec<String>>,
    /// (Output)
    /// Describes values that are relevant in a CA certificate.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "caOptions")]
    pub r#ca_options: Option<Box<super::super::types::certificateauthority::CertificateConfigX509ConfigCaOptions>>,
    /// (Output)
    /// Indicates the intended use for keys that correspond to a certificate.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "keyUsage")]
    pub r#key_usage: Box<super::super::types::certificateauthority::CertificateConfigX509ConfigKeyUsage>,
    /// (Output)
    /// Describes the X.509 name constraints extension.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "nameConstraints")]
    pub r#name_constraints: Option<Box<super::super::types::certificateauthority::CertificateConfigX509ConfigNameConstraints>>,
    /// (Output)
    /// Describes the X.509 certificate policy object identifiers, per https://tools.ietf.org/html/rfc5280#section-4.2.1.4.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "policyIds")]
    pub r#policy_ids: Option<Vec<super::super::types::certificateauthority::CertificateConfigX509ConfigPolicyId>>,
}
