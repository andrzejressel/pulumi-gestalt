#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MonitorUser {
    /// Country of the user.
    #[builder(into)]
    #[serde(rename = "country")]
    pub r#country: String,
    /// Email of the user used by Dynatrace for contacting them if needed.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: String,
    /// First name of the user.
    #[builder(into)]
    #[serde(rename = "firstName")]
    pub r#first_name: String,
    /// Last name of the user.
    #[builder(into)]
    #[serde(rename = "lastName")]
    pub r#last_name: String,
    /// phone number of the user by Dynatrace for contacting them if needed.
    #[builder(into)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: String,
}
