#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PermissionsBoundaryAttachmentPermissionsBoundary {
    /// Specifies the name and path of a customer managed policy. See below.
    #[builder(into)]
    #[serde(rename = "customerManagedPolicyReference")]
    pub r#customer_managed_policy_reference: Option<Box<super::super::types::ssoadmin::PermissionsBoundaryAttachmentPermissionsBoundaryCustomerManagedPolicyReference>>,
    /// AWS-managed IAM policy ARN to use as the permissions boundary.
    #[builder(into)]
    #[serde(rename = "managedPolicyArn")]
    pub r#managed_policy_arn: Option<String>,
}
