#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionResizeRequestStatusErrorErrorErrorDetailLocalizedMessage {
    /// (Output)
    /// The locale used following the specification defined at https://www.rfc-editor.org/rfc/bcp/bcp47.txt. Examples are: "en-US", "fr-CH", "es-MX"
    #[builder(into)]
    #[serde(rename = "locale")]
    pub r#locale: Option<String>,
    /// (Output)
    /// The localized error message in the above locale.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}
