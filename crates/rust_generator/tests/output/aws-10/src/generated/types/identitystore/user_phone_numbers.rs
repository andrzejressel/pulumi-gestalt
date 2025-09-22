#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserPhoneNumbers {
    /// When `true`, this is the primary phone number associated with the user.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Option<bool>,
    /// The type of phone number.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    /// The user's phone number.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
