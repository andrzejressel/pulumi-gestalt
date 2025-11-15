#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationPortalOptions {
    /// Sign-in options for the access portal. See `sign_in_options` below.
    #[builder(into)]
    #[serde(rename = "signInOptions")]
    pub r#sign_in_options: Option<Box<super::super::types::ssoadmin::ApplicationPortalOptionsSignInOptions>>,
    /// Indicates whether this application is visible in the access portal. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into)]
    #[serde(rename = "visibility")]
    pub r#visibility: Option<String>,
}
