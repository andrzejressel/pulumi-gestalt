#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SecurityScanConfigAuthentication {
    /// Describes authentication configuration that uses a custom account.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "customAccount")]
    pub r#custom_account: Box<Option<super::super::types::compute::SecurityScanConfigAuthenticationCustomAccount>>,
    /// Describes authentication configuration that uses a Google account.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "googleAccount")]
    pub r#google_account: Box<Option<super::super::types::compute::SecurityScanConfigAuthenticationGoogleAccount>>,
}
