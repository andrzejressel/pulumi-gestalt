#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRestApiEndpointConfiguration {
    #[builder(into)]
    #[serde(rename = "types")]
    pub r#types: Vec<String>,
    #[builder(into)]
    #[serde(rename = "vpcEndpointIds")]
    pub r#vpc_endpoint_ids: Vec<String>,
}
