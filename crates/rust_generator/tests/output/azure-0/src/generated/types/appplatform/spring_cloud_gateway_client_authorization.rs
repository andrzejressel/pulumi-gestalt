#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpringCloudGatewayClientAuthorization {
    /// Specifies the Spring Cloud Certificate IDs of the Spring Cloud Gateway.
    #[builder(into)]
    #[serde(rename = "certificateIds")]
    pub r#certificate_ids: Option<Vec<String>>,
    /// Specifies whether the client certificate verification is enabled.
    #[builder(into)]
    #[serde(rename = "verificationEnabled")]
    pub r#verification_enabled: Option<bool>,
}
