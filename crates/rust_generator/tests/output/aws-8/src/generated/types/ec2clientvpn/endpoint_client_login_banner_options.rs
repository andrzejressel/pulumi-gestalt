#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointClientLoginBannerOptions {
    /// Customizable text that will be displayed in a banner on AWS provided clients when a VPN session is established. UTF-8 encoded characters only. Maximum of 1400 characters.
    #[builder(into)]
    #[serde(rename = "bannerText")]
    pub r#banner_text: Option<String>,
    /// Enable or disable a customizable text banner that will be displayed on AWS provided clients when a VPN session is established. The default is `false` (not enabled).
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
}
