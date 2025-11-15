#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelDestinationSetting {
    /// Key used to extract the password from EC2 Parameter store.
    #[builder(into)]
    #[serde(rename = "passwordParam")]
    pub r#password_param: Option<String>,
    /// Stream name RTMP destinations (URLs of type rtmp://)
    #[builder(into)]
    #[serde(rename = "streamName")]
    pub r#stream_name: Option<String>,
    /// A URL specifying a destination.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
    /// Username for destination.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Option<String>,
}
