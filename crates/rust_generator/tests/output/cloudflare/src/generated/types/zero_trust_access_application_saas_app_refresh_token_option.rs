#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustAccessApplicationSaasAppRefreshTokenOption {
    /// How long a refresh token will be valid for after creation. Valid units are `m`, `h` and `d`. Must be longer than 1m.
    #[builder(into)]
    #[serde(rename = "lifetime")]
    pub r#lifetime: Option<String>,
}
