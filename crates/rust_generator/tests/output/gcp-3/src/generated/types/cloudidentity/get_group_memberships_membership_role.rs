#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetGroupMembershipsMembershipRole {
    /// The MembershipRole expiry details, only supported for MEMBER role.
    /// Other roles cannot be accompanied with MEMBER role having expiry.
    #[builder(into)]
    #[serde(rename = "expiryDetails")]
    pub r#expiry_details: Vec<super::super::types::cloudidentity::GetGroupMembershipsMembershipRoleExpiryDetail>,
    /// The name of the MembershipRole. One of OWNER, MANAGER, MEMBER.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
