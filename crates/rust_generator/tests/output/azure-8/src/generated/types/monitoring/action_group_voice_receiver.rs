#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ActionGroupVoiceReceiver {
    /// The country code of the voice receiver.
    #[builder(into)]
    #[serde(rename = "countryCode")]
    pub r#country_code: Box<String>,
    /// The name of the voice receiver.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The phone number of the voice receiver.
    #[builder(into)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: Box<String>,
}
