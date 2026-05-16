#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketIntelligentTieringConfigurationTiering {
    /// S3 Intelligent-Tiering access tier. Valid values: `ARCHIVE_ACCESS`, `DEEP_ARCHIVE_ACCESS`.
    #[builder(into)]
    #[serde(rename = "accessTier")]
    pub r#access_tier: String,
    /// Number of consecutive days of no access after which an object will be eligible to be transitioned to the corresponding tier.
    #[builder(into)]
    #[serde(rename = "days")]
    pub r#days: i32,
}
