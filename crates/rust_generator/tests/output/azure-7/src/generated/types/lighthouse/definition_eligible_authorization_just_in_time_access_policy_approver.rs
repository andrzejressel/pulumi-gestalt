#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DefinitionEligibleAuthorizationJustInTimeAccessPolicyApprover {
    /// The display name of the Azure Active Directory Principal for the approver.
    #[builder(into)]
    #[serde(rename = "principalDisplayName")]
    pub r#principal_display_name: Option<String>,
    /// The Principal ID of the Azure Active Directory principal for the approver.
    #[builder(into)]
    #[serde(rename = "principalId")]
    pub r#principal_id: String,
}
