#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationAzureSecurityBaseline {
    /// The assignment type of the azure security baseline. Possible values are `ApplyAndAutoCorrect`, `ApplyAndMonitor`, `Audit` and `DeployAndAutoCorrect`. Defaults to `ApplyAndAutoCorrect`.
    #[builder(into)]
    #[serde(rename = "assignmentType")]
    pub r#assignment_type: Option<String>,
}
