#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetListenerMutualAuthentication {
    #[builder(into)]
    #[serde(rename = "advertiseTrustStoreCaNames")]
    pub r#advertise_trust_store_ca_names: String,
    #[builder(into)]
    #[serde(rename = "ignoreClientCertificateExpiry")]
    pub r#ignore_client_certificate_expiry: bool,
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
    #[builder(into)]
    #[serde(rename = "trustStoreArn")]
    pub r#trust_store_arn: String,
}
