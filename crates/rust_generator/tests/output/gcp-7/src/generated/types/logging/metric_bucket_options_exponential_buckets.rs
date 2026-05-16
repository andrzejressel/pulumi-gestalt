#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MetricBucketOptionsExponentialBuckets {
    /// Must be greater than 1.
    #[builder(into)]
    #[serde(rename = "growthFactor")]
    pub r#growth_factor: f64,
    /// Must be greater than 0.
    #[builder(into)]
    #[serde(rename = "numFiniteBuckets")]
    pub r#num_finite_buckets: i32,
    /// Must be greater than 0.
    #[builder(into)]
    #[serde(rename = "scale")]
    pub r#scale: f64,
}
