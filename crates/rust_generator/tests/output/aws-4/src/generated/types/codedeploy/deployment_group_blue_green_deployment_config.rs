#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeploymentGroupBlueGreenDeploymentConfig {
    /// Information about the action to take when newly provisioned instances are ready to receive traffic in a blue/green deployment (documented below).
    #[builder(into)]
    #[serde(rename = "deploymentReadyOption")]
    pub r#deployment_ready_option: Option<Box<super::super::types::codedeploy::DeploymentGroupBlueGreenDeploymentConfigDeploymentReadyOption>>,
    /// Information about how instances are provisioned for a replacement environment in a blue/green deployment (documented below).
    #[builder(into)]
    #[serde(rename = "greenFleetProvisioningOption")]
    pub r#green_fleet_provisioning_option: Option<Box<super::super::types::codedeploy::DeploymentGroupBlueGreenDeploymentConfigGreenFleetProvisioningOption>>,
    /// Information about whether to terminate instances in the original fleet during a blue/green deployment (documented below).
    /// 
    /// _Only one `blue_green_deployment_config` is allowed_.
    #[builder(into)]
    #[serde(rename = "terminateBlueInstancesOnDeploymentSuccess")]
    pub r#terminate_blue_instances_on_deployment_success: Option<Box<super::super::types::codedeploy::DeploymentGroupBlueGreenDeploymentConfigTerminateBlueInstancesOnDeploymentSuccess>>,
}
