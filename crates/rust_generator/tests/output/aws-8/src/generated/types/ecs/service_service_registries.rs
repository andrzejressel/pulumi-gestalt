#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceServiceRegistries {
    /// Container name value, already specified in the task definition, to be used for your service discovery service.
    #[builder(into)]
    #[serde(rename = "containerName")]
    pub r#container_name: Option<String>,
    /// Port value, already specified in the task definition, to be used for your service discovery service.
    #[builder(into)]
    #[serde(rename = "containerPort")]
    pub r#container_port: Option<i32>,
    /// Port value used if your Service Discovery service specified an SRV record.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// ARN of the Service Registry. The currently supported service registry is Amazon Route 53 Auto Naming Service(`aws.servicediscovery.Service`). For more information, see [Service](https://docs.aws.amazon.com/Route53/latest/APIReference/API_autonaming_Service.html)
    #[builder(into)]
    #[serde(rename = "registryArn")]
    pub r#registry_arn: String,
}
