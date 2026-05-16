#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointPolicyEndpointMatcherMetadataLabelMatcherMetadataLabel {
    /// Required. Label name presented as key in xDS Node Metadata.
    #[builder(into)]
    #[serde(rename = "labelName")]
    pub r#label_name: String,
    /// Required. Label value presented as value corresponding to the above key, in xDS Node Metadata.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "labelValue")]
    pub r#label_value: String,
}
