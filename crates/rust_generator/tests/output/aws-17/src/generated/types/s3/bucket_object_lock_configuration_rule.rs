#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketObjectLockConfigurationRule {
    /// The default retention period that you want to apply to new objects placed in this bucket.
    #[builder(into)]
    #[serde(rename = "defaultRetention")]
    pub r#default_retention: Box<super::super::types::s3::BucketObjectLockConfigurationRuleDefaultRetention>,
}
