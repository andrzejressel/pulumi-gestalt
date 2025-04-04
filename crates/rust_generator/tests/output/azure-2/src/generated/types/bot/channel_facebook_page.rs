#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelFacebookPage {
    /// The Facebook Page Access Token for the Facebook Channel.
    #[builder(into)]
    #[serde(rename = "accessToken")]
    pub r#access_token: Box<String>,
    /// The Facebook Page ID for the Facebook Channel.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
}
