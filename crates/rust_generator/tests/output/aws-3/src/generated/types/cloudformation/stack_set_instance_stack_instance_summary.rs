#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StackSetInstanceStackInstanceSummary {
    /// Target AWS Account ID to create a Stack based on the StackSet. Defaults to current account.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: Option<String>,
    /// Organizational unit ID in which the stack is deployed.
    #[builder(into)]
    #[serde(rename = "organizationalUnitId")]
    pub r#organizational_unit_id: Option<String>,
    /// Stack identifier.
    #[builder(into)]
    #[serde(rename = "stackId")]
    pub r#stack_id: Option<String>,
}
