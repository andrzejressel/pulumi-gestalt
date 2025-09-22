#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MembershipRbacRoleBindingState {
    /// (Output)
    /// Code describes the state of a RBAC Role Binding resource.
    #[builder(into)]
    #[serde(rename = "code")]
    pub r#code: Option<String>,
}
