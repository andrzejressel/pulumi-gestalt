#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EmailTemplateEmailTemplate {
    /// JSON object that specifies the default values to use for message variables in the message template. This object is a set of key-value pairs. Each key defines a message variable in the template. The corresponding value defines the default value for that variable. When you create a message that's based on the template, you can override these defaults with message-specific and address-specific variables and values.
    #[builder(into)]
    #[serde(rename = "defaultSubstitutions")]
    pub r#default_substitutions: Option<String>,
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<super::super::types::pinpoint::EmailTemplateEmailTemplateHeader>>,
    /// The message body, in HTML format, to use in email messages that are based on the message template. We recommend using HTML format for email clients that render HTML content. You can include links, formatted text, and more in an HTML message.
    #[builder(into)]
    #[serde(rename = "htmlPart")]
    pub r#html_part: Option<String>,
    /// The unique identifier for the recommender model to use for the message template. Amazon Pinpoint uses this value to determine how to retrieve and process data from a recommender model when it sends messages that use the template, if the template contains message variables for recommendation data.
    #[builder(into)]
    #[serde(rename = "recommenderId")]
    pub r#recommender_id: Option<String>,
    /// Subject line, or title, to use in email messages that are based on the message template.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Option<String>,
    /// Message body, in plain text format, to use in email messages that are based on the message template. We recommend using plain text format for email clients that don't render HTML content and clients that are connected to high-latency networks, such as mobile devices.
    #[builder(into)]
    #[serde(rename = "textPart")]
    pub r#text_part: Option<String>,
}
