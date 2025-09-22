#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionBackendServiceStrongSessionAffinityCookie {
    /// Name of the cookie.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Path to set for the cookie.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Lifetime of the cookie.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<Option<super::super::types::compute::RegionBackendServiceStrongSessionAffinityCookieTtl>>,
}
