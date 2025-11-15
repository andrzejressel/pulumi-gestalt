#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KeyRingImportJobAttestation {
    /// (Output)
    /// The attestation data provided by the HSM when the key operation was performed.
    /// A base64-encoded string.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// (Output)
    /// The format of the attestation data.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: Option<String>,
}
