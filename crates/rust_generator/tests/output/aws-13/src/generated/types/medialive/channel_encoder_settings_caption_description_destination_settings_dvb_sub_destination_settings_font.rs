#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ChannelEncoderSettingsCaptionDescriptionDestinationSettingsDvbSubDestinationSettingsFont {
    /// Key used to extract the password from EC2 Parameter store.
    #[builder(into, default)]
    #[serde(rename = "passwordParam")]
    pub r#password_param: Box<Option<String>>,
    /// Path to a file accessible to the live stream.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
    /// Username to be used.
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
