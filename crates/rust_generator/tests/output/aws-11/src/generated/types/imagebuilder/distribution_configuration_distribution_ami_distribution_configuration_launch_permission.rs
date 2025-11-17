#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionConfigurationDistributionAmiDistributionConfigurationLaunchPermission {
    /// Set of AWS Organization ARNs to assign.
    #[builder(into)]
    #[serde(rename = "organizationArns")]
    pub r#organization_arns: Option<Vec<String>>,
    /// Set of AWS Organizational Unit ARNs to assign.
    #[builder(into)]
    #[serde(rename = "organizationalUnitArns")]
    pub r#organizational_unit_arns: Option<Vec<String>>,
    /// Set of EC2 launch permission user groups to assign. Use `all` to distribute a public AMI.
    #[builder(into)]
    #[serde(rename = "userGroups")]
    pub r#user_groups: Option<Vec<String>>,
    /// Set of AWS Account identifiers to assign.
    #[builder(into)]
    #[serde(rename = "userIds")]
    pub r#user_ids: Option<Vec<String>>,
}
