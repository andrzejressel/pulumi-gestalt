#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DeploymentGroupBlueGreenDeploymentConfigTerminateBlueInstancesOnDeploymentSuccess {
    /// The action to take on instances in the original environment after a successful blue/green deployment.
    /// * `TERMINATE`: Instances are terminated after a specified wait time.
    /// * `KEEP_ALIVE`: Instances are left running after they are deregistered from the load balancer and removed from the deployment group.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Option<String>,
    /// The number of minutes to wait after a successful blue/green deployment before terminating instances from the original environment.
    #[builder(into)]
    #[serde(rename = "terminationWaitTimeInMinutes")]
    pub r#termination_wait_time_in_minutes: Option<i32>,
}
