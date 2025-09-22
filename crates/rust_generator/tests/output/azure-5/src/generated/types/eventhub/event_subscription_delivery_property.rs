#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EventSubscriptionDeliveryProperty {
    /// The name of the header to send on to the destination
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: String,
    /// True if the `value` is a secret and should be protected, otherwise false. If True, then this value won't be returned from Azure API calls
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Option<bool>,
    /// If the `type` is `Dynamic`, then provide the payload field to be used as the value. Valid source fields differ by subscription type.
    #[builder(into)]
    #[serde(rename = "sourceField")]
    pub r#source_field: Option<String>,
    /// Either `Static` or `Dynamic`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
    /// If the `type` is `Static`, then provide the value to use
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
