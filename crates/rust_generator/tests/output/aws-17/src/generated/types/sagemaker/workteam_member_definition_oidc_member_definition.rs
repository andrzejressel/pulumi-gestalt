#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WorkteamMemberDefinitionOidcMemberDefinition {
    /// A list of comma separated strings that identifies user groups in your OIDC IdP. Each user group is made up of a group of private workers.
    #[builder(into)]
    #[serde(rename = "groups")]
    pub r#groups: Vec<String>,
}
