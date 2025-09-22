#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RecordSetRoutingPolicyPrimaryBackupPrimaryInternalLoadBalancer {
    /// The frontend IP address of the load balancer.
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: String,
    /// The configured IP protocol of the load balancer. This value is case-sensitive. Possible values: ["tcp", "udp"]
    #[builder(into)]
    #[serde(rename = "ipProtocol")]
    pub r#ip_protocol: String,
    /// The type of load balancer. This value is case-sensitive. Possible values: ["regionalL4ilb", "regionalL7ilb", "globalL7ilb"]
    #[builder(into)]
    #[serde(rename = "loadBalancerType")]
    pub r#load_balancer_type: Option<String>,
    /// The fully qualified url of the network in which the load balancer belongs. This should be formatted like `projects/{project}/global/networks/{network}` or `https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}`.
    #[builder(into)]
    #[serde(rename = "networkUrl")]
    pub r#network_url: String,
    /// The configured port of the load balancer.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: String,
    /// The ID of the project in which the load balancer belongs.
    #[builder(into)]
    #[serde(rename = "project")]
    pub r#project: String,
    /// The region of the load balancer. Only needed for regional load balancers.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
}
