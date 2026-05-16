#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterOptimizedAutoScale {
    /// The maximum number of allowed instances. Must between `0` and `1000`.
    #[builder(into)]
    #[serde(rename = "maximumInstances")]
    pub r#maximum_instances: i32,
    /// The minimum number of allowed instances. Must between `0` and `1000`.
    #[builder(into)]
    #[serde(rename = "minimumInstances")]
    pub r#minimum_instances: i32,
}
