#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetAuthorityConfigX509ConfigKeyUsage {
    /// Describes high-level ways in which a key may be used.
    #[builder(into)]
    #[serde(rename = "baseKeyUsages")]
    pub r#base_key_usages: Box<Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigKeyUsageBaseKeyUsage>>,
    /// Describes high-level ways in which a key may be used.
    #[builder(into)]
    #[serde(rename = "extendedKeyUsages")]
    pub r#extended_key_usages: Box<Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigKeyUsageExtendedKeyUsage>>,
    /// An ObjectId specifies an object identifier (OID). These provide context and describe types in ASN.1 messages.
    #[builder(into)]
    #[serde(rename = "unknownExtendedKeyUsages")]
    pub r#unknown_extended_key_usages: Box<Vec<super::super::types::certificateauthority::GetAuthorityConfigX509ConfigKeyUsageUnknownExtendedKeyUsage>>,
}
