#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeploymentGroupEcsService {
    /// The name of the ECS cluster.
    #[builder(into)]
    #[serde(rename = "clusterName")]
    pub r#cluster_name: String,
    /// The name of the ECS service.
    #[builder(into)]
    #[serde(rename = "serviceName")]
    pub r#service_name: String,
}
