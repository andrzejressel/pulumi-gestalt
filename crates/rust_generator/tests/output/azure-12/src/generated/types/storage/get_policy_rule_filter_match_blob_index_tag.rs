#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPolicyRuleFilterMatchBlobIndexTag {
    /// The filter tag name used for tag based filtering for blob objects.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The comparison operator which is used for object comparison and filtering. Possible value is `==`. Defaults to `==`.
    #[builder(into)]
    #[serde(rename = "operation")]
    pub r#operation: String,
    /// The filter tag value used for tag based filtering for blob objects.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: String,
}
