#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegistryTaskSourceTrigger {
    /// A `authentication` block as defined above.
    #[builder(into)]
    #[serde(rename = "authentication")]
    pub r#authentication: Option<Box<super::super::types::containerservice::RegistryTaskSourceTriggerAuthentication>>,
    /// The branch name of the source code.
    #[builder(into)]
    #[serde(rename = "branch")]
    pub r#branch: Option<String>,
    /// Should the trigger be enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Specifies a list of source events corresponding to the trigger. Possible values are `commit` and `pullrequest`.
    #[builder(into)]
    #[serde(rename = "events")]
    pub r#events: Vec<String>,
    /// The name which should be used for this trigger.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The full URL to the source code repository.
    #[builder(into)]
    #[serde(rename = "repositoryUrl")]
    pub r#repository_url: String,
    /// The type of the source control service. Possible values are `Github` and `VisualStudioTeamService`.
    #[builder(into)]
    #[serde(rename = "sourceType")]
    pub r#source_type: String,
}
