#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ListenerMutualAuthentication {
    /// Valid values are `off` and `on`.
    #[builder(into)]
    #[serde(rename = "advertiseTrustStoreCaNames")]
    pub r#advertise_trust_store_ca_names: Option<String>,
    /// Whether client certificate expiry is ignored. Default is `false`.
    #[builder(into)]
    #[serde(rename = "ignoreClientCertificateExpiry")]
    pub r#ignore_client_certificate_expiry: Option<bool>,
    /// Valid values are `off`, `verify` and `passthrough`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
    /// ARN of the elbv2 Trust Store.
    #[builder(into)]
    #[serde(rename = "trustStoreArn")]
    pub r#trust_store_arn: Option<String>,
}
