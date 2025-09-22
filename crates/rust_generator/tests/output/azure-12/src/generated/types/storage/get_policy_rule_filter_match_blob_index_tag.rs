#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
