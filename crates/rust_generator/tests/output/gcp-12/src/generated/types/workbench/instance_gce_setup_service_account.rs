#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceGceSetupServiceAccount {
    /// Optional. Email address of the service account.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// (Output)
    /// Output only. The list of scopes to be made available for this
    /// service account. Set by the CLH to https://www.googleapis.com/auth/cloud-platform
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<String>>,
}
