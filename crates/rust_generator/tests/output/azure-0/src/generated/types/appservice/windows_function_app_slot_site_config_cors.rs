#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsFunctionAppSlotSiteConfigCors {
    /// Specifies a list of origins that should be allowed to make cross-origin calls.
    #[builder(into)]
    #[serde(rename = "allowedOrigins")]
    pub r#allowed_origins: Option<Vec<String>>,
    /// Are credentials allowed in CORS requests? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "supportCredentials")]
    pub r#support_credentials: Option<bool>,
}
