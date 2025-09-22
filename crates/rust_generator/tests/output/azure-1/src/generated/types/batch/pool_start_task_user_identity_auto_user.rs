#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolStartTaskUserIdentityAutoUser {
    /// The elevation level of the user identity under which the start task runs. Possible values are `Admin` or `NonAdmin`. Defaults to `NonAdmin`.
    #[builder(into)]
    #[serde(rename = "elevationLevel")]
    pub r#elevation_level: Option<String>,
    /// The scope of the user identity under which the start task runs. Possible values are `Task` or `Pool`. Defaults to `Task`.
    #[builder(into)]
    #[serde(rename = "scope")]
    pub r#scope: Option<String>,
}
