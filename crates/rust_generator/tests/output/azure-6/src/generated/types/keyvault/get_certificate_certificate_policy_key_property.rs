#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetCertificateCertificatePolicyKeyProperty {
    #[builder(into)]
    #[serde(rename = "curve")]
    pub r#curve: String,
    /// Is this Certificate Exportable?
    #[builder(into)]
    #[serde(rename = "exportable")]
    pub r#exportable: bool,
    /// The size of the Key used in the Certificate.
    #[builder(into)]
    #[serde(rename = "keySize")]
    pub r#key_size: i32,
    /// Specifies the Type of Key, for example `RSA`.
    #[builder(into)]
    #[serde(rename = "keyType")]
    pub r#key_type: String,
    /// Is the key reusable?
    #[builder(into)]
    #[serde(rename = "reuseKey")]
    pub r#reuse_key: bool,
}
