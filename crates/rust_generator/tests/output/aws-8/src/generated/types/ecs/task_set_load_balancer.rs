#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct TaskSetLoadBalancer {
    /// The name of the container to associate with the load balancer (as it appears in a container definition).
    #[builder(into)]
    #[serde(rename = "containerName")]
    pub r#container_name: String,
    /// The port on the container to associate with the load balancer. Defaults to `0` if not specified.
    /// 
    /// > **Note:** Specifying multiple `load_balancer` configurations is still not supported by AWS for ECS task set.
    #[builder(into)]
    #[serde(rename = "containerPort")]
    pub r#container_port: Option<i32>,
    /// The name of the ELB (Classic) to associate with the service.
    #[builder(into)]
    #[serde(rename = "loadBalancerName")]
    pub r#load_balancer_name: Option<String>,
    /// The ARN of the Load Balancer target group to associate with the service.
    #[builder(into)]
    #[serde(rename = "targetGroupArn")]
    pub r#target_group_arn: Option<String>,
}
