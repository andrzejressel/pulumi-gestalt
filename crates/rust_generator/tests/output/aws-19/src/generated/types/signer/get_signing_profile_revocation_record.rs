#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetSigningProfileRevocationRecord {
    #[builder(into)]
    #[serde(rename = "revocationEffectiveFrom")]
    pub r#revocation_effective_from: String,
    #[builder(into)]
    #[serde(rename = "revokedAt")]
    pub r#revoked_at: String,
    #[builder(into)]
    #[serde(rename = "revokedBy")]
    pub r#revoked_by: String,
}
