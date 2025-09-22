#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CertificateTemplatePassthroughExtensions {
    /// Optional. A set of ObjectIds identifying custom X.509 extensions. Will be combined with known_extensions to determine the full set of X.509 extensions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "additionalExtensions")]
    pub r#additional_extensions: Option<Vec<super::super::types::certificateauthority::CertificateTemplatePassthroughExtensionsAdditionalExtension>>,
    /// Optional. A set of named X.509 extensions. Will be combined with additional_extensions to determine the full set of X.509 extensions.
    #[builder(into)]
    #[serde(rename = "knownExtensions")]
    pub r#known_extensions: Option<Vec<String>>,
}
