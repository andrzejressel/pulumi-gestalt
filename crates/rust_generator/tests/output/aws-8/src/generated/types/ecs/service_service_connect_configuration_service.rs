#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceServiceConnectConfigurationService {
    /// List of client aliases for this Service Connect service. You use these to assign names that can be used by client applications. The maximum number of client aliases that you can have in this list is 1. See below.
    #[builder(into)]
    #[serde(rename = "clientAlias")]
    pub r#client_alias: Option<Vec<super::super::types::ecs::ServiceServiceConnectConfigurationServiceClientAlias>>,
    /// Name of the new AWS Cloud Map service that Amazon ECS creates for this Amazon ECS service.
    #[builder(into)]
    #[serde(rename = "discoveryName")]
    pub r#discovery_name: Option<String>,
    /// Port number for the Service Connect proxy to listen on.
    #[builder(into)]
    #[serde(rename = "ingressPortOverride")]
    pub r#ingress_port_override: Option<i32>,
    /// Name of one of the `portMappings` from all the containers in the task definition of this Amazon ECS service.
    #[builder(into)]
    #[serde(rename = "portName")]
    pub r#port_name: String,
    /// Configuration timeouts for Service Connect
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<Box<super::super::types::ecs::ServiceServiceConnectConfigurationServiceTimeout>>,
    /// Configuration for enabling Transport Layer Security (TLS)
    #[builder(into)]
    #[serde(rename = "tls")]
    pub r#tls: Option<Box<super::super::types::ecs::ServiceServiceConnectConfigurationServiceTls>>,
}
