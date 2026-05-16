#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceEndpointSpec {
    /// The mode of resolution to use for internal load balancing between tasks
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<String>,
    /// List of exposed ports that this service is accessible on from the outside. Ports can only be provided if 'vip' resolution mode is used
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Option<Vec<super::types::ServiceEndpointSpecPort>>,
}
