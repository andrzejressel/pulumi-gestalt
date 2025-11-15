#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2VmNetworkEndpoint {
    /// (Output)
    /// The access config for the TPU worker.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "accessConfigs")]
    pub r#access_configs: Option<Vec<super::super::types::tpu::V2VmNetworkEndpointAccessConfig>>,
    /// (Output)
    /// The internal IP address of this network endpoint.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Option<String>,
    /// (Output)
    /// The port of this network endpoint.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
}
