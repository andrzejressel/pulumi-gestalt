#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EndpointDeploymentConfigBlueGreenUpdatePolicy {
    #[builder(into)]
    #[serde(rename = "maximumExecutionTimeoutInSeconds")]
    pub r#maximum_execution_timeout_in_seconds: Option<i32>,
    /// Additional waiting time in seconds after the completion of an endpoint deployment before terminating the old endpoint fleet. Default is `0`. Valid values are between `0` and `3600`.
    #[builder(into)]
    #[serde(rename = "terminationWaitInSeconds")]
    pub r#termination_wait_in_seconds: Option<i32>,
    /// Defines the traffic routing strategy to shift traffic from the old fleet to the new fleet during an endpoint deployment. See Traffic Routing Configuration.
    #[builder(into)]
    #[serde(rename = "trafficRoutingConfiguration")]
    pub r#traffic_routing_configuration: Box<super::super::types::sagemaker::EndpointDeploymentConfigBlueGreenUpdatePolicyTrafficRoutingConfiguration>,
}
