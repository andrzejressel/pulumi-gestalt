#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionLocation {
    /// The Availability Zone. Follows the format us-east-2a (case-sensitive).
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: String,
    /// The AWS Region name.
    #[builder(into)]
    #[serde(rename = "regionName")]
    pub r#region_name: String,
}
