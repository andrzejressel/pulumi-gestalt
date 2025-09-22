#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct V2VmServiceAccount {
    /// Email address of the service account. If empty, default Compute service account will be used.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// The list of scopes to be made available for this service account. If empty, access to all
    /// Cloud APIs will be allowed.
    #[builder(into)]
    #[serde(rename = "scopes")]
    pub r#scopes: Option<Vec<String>>,
}
