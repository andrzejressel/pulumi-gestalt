#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClassificationJobS3JobDefinitionBucketDefinition {
    /// The unique identifier for the AWS account that owns the buckets.
    #[builder(into)]
    #[serde(rename = "accountId")]
    pub r#account_id: String,
    /// An array that lists the names of the buckets.
    #[builder(into)]
    #[serde(rename = "buckets")]
    pub r#buckets: Vec<String>,
}
