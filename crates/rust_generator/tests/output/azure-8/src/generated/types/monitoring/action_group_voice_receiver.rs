#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ActionGroupVoiceReceiver {
    /// The country code of the voice receiver.
    #[builder(into)]
    #[serde(rename = "countryCode")]
    pub r#country_code: String,
    /// The name of the voice receiver.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The phone number of the voice receiver.
    #[builder(into)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: String,
}
