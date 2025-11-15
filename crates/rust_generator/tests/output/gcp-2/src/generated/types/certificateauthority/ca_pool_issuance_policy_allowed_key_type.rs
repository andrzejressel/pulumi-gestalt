#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CaPoolIssuancePolicyAllowedKeyType {
    /// Represents an allowed Elliptic Curve key type.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ellipticCurve")]
    pub r#elliptic_curve: Option<Box<super::super::types::certificateauthority::CaPoolIssuancePolicyAllowedKeyTypeEllipticCurve>>,
    /// Describes an RSA key that may be used in a Certificate issued from a CaPool.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "rsa")]
    pub r#rsa: Option<Box<super::super::types::certificateauthority::CaPoolIssuancePolicyAllowedKeyTypeRsa>>,
}
