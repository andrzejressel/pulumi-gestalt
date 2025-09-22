#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointPolicyEndpointMatcherMetadataLabelMatcher {
    /// Specifies how matching should be done.
    /// Possible values are: `MATCH_ANY`, `MATCH_ALL`.
    #[builder(into)]
    #[serde(rename = "metadataLabelMatchCriteria")]
    pub r#metadata_label_match_criteria: String,
    /// The list of label value pairs that must match labels in the provided metadata based on filterMatchCriteria
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "metadataLabels")]
    pub r#metadata_labels: Option<Vec<super::super::types::networkservices::EndpointPolicyEndpointMatcherMetadataLabelMatcherMetadataLabel>>,
}
