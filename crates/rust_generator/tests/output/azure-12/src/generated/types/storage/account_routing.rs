#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountRouting {
    /// Specifies the kind of network routing opted by the user. Possible values are `InternetRouting` and `MicrosoftRouting`. Defaults to `MicrosoftRouting`.
    #[builder(into)]
    #[serde(rename = "choice")]
    pub r#choice: Option<String>,
    /// Should internet routing storage endpoints be published? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "publishInternetEndpoints")]
    pub r#publish_internet_endpoints: Option<bool>,
    /// Should Microsoft routing storage endpoints be published? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "publishMicrosoftEndpoints")]
    pub r#publish_microsoft_endpoints: Option<bool>,
}
