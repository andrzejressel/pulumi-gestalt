#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TriggerBuildAvailableSecrets {
    /// Pairs a secret environment variable with a SecretVersion in Secret Manager.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "secretManagers")]
    pub r#secret_managers: Vec<super::super::types::cloudbuild::TriggerBuildAvailableSecretsSecretManager>,
}
