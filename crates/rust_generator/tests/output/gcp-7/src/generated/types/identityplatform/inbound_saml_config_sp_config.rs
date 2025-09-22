#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InboundSamlConfigSpConfig {
    /// Callback URI where responses from IDP are handled. Must start with `https://`.
    #[builder(into)]
    #[serde(rename = "callbackUri")]
    pub r#callback_uri: Option<String>,
    /// (Output)
    /// The IDP's certificate data to verify the signature in the SAMLResponse issued by the IDP.
    /// Structure is documented below.
    /// 
    /// 
    /// <a name="nested_sp_certificates"></a>The `sp_certificates` block contains:
    #[builder(into)]
    #[serde(rename = "spCertificates")]
    pub r#sp_certificates: Option<Vec<super::super::types::identityplatform::InboundSamlConfigSpConfigSpCertificate>>,
    /// Unique identifier for all SAML entities.
    #[builder(into)]
    #[serde(rename = "spEntityId")]
    pub r#sp_entity_id: Option<String>,
}
