#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RecordGeoproximityRoutingPolicy {
    /// A AWS region where the resource is present.
    #[builder(into)]
    #[serde(rename = "awsRegion")]
    pub r#aws_region: Option<String>,
    /// Route more traffic or less traffic to the resource by specifying a value ranges between -90 to 90. See https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-policy-geoproximity.html for bias details.
    #[builder(into)]
    #[serde(rename = "bias")]
    pub r#bias: Option<i32>,
    /// Specify `latitude` and `longitude` for routing traffic to non-AWS resources.
    #[builder(into)]
    #[serde(rename = "coordinates")]
    pub r#coordinates: Option<Vec<super::super::types::route53::RecordGeoproximityRoutingPolicyCoordinate>>,
    /// A AWS local zone group where the resource is present. See https://docs.aws.amazon.com/local-zones/latest/ug/available-local-zones.html for local zone group list.
    #[builder(into)]
    #[serde(rename = "localZoneGroup")]
    pub r#local_zone_group: Option<String>,
}
