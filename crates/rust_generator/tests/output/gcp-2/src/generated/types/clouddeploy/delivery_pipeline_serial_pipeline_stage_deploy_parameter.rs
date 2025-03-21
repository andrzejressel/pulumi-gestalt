#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeliveryPipelineSerialPipelineStageDeployParameter {
    /// Optional. Deploy parameters are applied to targets with match labels. If unspecified, deploy parameters are applied to all targets (including child targets of a multi-target).
    #[builder(into, default)]
    #[serde(rename = "matchTargetLabels")]
    pub r#match_target_labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Required. Values are deploy parameters in key-value pairs.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<std::collections::HashMap<String, String>>,
}
