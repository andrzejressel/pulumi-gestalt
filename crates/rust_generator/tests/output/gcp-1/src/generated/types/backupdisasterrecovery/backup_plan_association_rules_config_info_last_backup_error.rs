#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackupPlanAssociationRulesConfigInfoLastBackupError {
    /// (Output)
    /// The status code, which should be an enum value of [google.rpc.Code]
    #[builder(into)]
    #[serde(rename = "code")]
    pub r#code: Option<f64>,
    /// (Output)
    /// A developer-facing error message, which should be in English.
    #[builder(into)]
    #[serde(rename = "message")]
    pub r#message: Option<String>,
}
