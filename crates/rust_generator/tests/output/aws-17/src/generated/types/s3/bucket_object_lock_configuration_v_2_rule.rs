#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketObjectLockConfigurationV2Rule {
    /// Configuration block for specifying the default Object Lock retention settings for new objects placed in the specified bucket. See below.
    #[builder(into)]
    #[serde(rename = "defaultRetention")]
    pub r#default_retention: Box<super::super::types::s3::BucketObjectLockConfigurationV2RuleDefaultRetention>,
}
