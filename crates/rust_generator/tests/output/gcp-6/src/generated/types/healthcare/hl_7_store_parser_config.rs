#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct Hl7StoreParserConfig {
    /// Determines whether messages with no header are allowed.
    #[builder(into)]
    #[serde(rename = "allowNullHeader")]
    pub r#allow_null_header: Option<bool>,
    /// JSON encoded string for schemas used to parse messages in this
    /// store if schematized parsing is desired.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Option<String>,
    /// Byte(s) to be used as the segment terminator. If this is unset, '\r' will be used as segment terminator.
    /// A base64-encoded string.
    #[builder(into)]
    #[serde(rename = "segmentTerminator")]
    pub r#segment_terminator: Option<String>,
    /// The version of the unschematized parser to be used when a custom `schema` is not set.
    /// Default value is `V1`.
    /// Possible values are: `V1`, `V2`, `V3`.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
