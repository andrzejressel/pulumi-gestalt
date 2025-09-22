#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NoteAttestationAuthorityHint {
    /// The human readable name of this Attestation Authority, for
    /// example "qa".
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "humanReadableName")]
    pub r#human_readable_name: String,
}
