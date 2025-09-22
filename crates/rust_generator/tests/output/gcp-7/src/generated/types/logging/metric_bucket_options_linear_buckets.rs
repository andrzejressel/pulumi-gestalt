#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MetricBucketOptionsLinearBuckets {
    /// Must be greater than 0.
    #[builder(into)]
    #[serde(rename = "numFiniteBuckets")]
    pub r#num_finite_buckets: i32,
    /// Lower bound of the first bucket.
    #[builder(into)]
    #[serde(rename = "offset")]
    pub r#offset: f64,
    /// Must be greater than 0.
    #[builder(into)]
    #[serde(rename = "width")]
    pub r#width: f64,
}
