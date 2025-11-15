#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResourceSetResourceDnsTargetResource {
    /// DNS Name that acts as the ingress point to a portion of application.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: String,
    /// Hosted Zone ARN that contains the DNS record with the provided name of target resource.
    #[builder(into)]
    #[serde(rename = "hostedZoneArn")]
    pub r#hosted_zone_arn: Option<String>,
    /// Route53 record set id to uniquely identify a record given a `domain_name` and a `record_type`.
    #[builder(into)]
    #[serde(rename = "recordSetId")]
    pub r#record_set_id: Option<String>,
    /// Type of DNS Record of target resource.
    #[builder(into)]
    #[serde(rename = "recordType")]
    pub r#record_type: Option<String>,
    /// Target resource the R53 record specified with the above params points to.
    #[builder(into)]
    #[serde(rename = "targetResource")]
    pub r#target_resource: Option<Box<super::super::types::route53recoveryreadiness::ResourceSetResourceDnsTargetResourceTargetResource>>,
}
