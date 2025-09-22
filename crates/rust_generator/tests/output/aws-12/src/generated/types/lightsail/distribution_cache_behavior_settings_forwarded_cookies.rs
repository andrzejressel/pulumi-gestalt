#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DistributionCacheBehaviorSettingsForwardedCookies {
    /// The specific cookies to forward to your distribution's origin.
    #[builder(into)]
    #[serde(rename = "cookiesAllowLists")]
    pub r#cookies_allow_lists: Option<Vec<String>>,
    /// Specifies which cookies to forward to the distribution's origin for a cache behavior: all, none, or allow-list to forward only the cookies specified in the cookiesAllowList parameter.
    #[builder(into)]
    #[serde(rename = "option")]
    pub r#option: Option<String>,
}
