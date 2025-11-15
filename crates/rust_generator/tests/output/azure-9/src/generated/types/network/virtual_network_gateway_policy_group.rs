#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VirtualNetworkGatewayPolicyGroup {
    /// Is this a Default Virtual Network Gateway Policy Group? Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "isDefault")]
    pub r#is_default: Option<bool>,
    /// The name of the Virtual Network Gateway Policy Group.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// One or more `policy_member` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "policyMembers")]
    pub r#policy_members: Vec<super::super::types::network::VirtualNetworkGatewayPolicyGroupPolicyMember>,
    /// The priority for the Virtual Network Gateway Policy Group. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Option<i32>,
}
