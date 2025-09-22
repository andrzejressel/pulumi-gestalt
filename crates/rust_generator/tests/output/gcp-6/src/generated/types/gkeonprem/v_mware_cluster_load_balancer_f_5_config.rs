#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VMwareClusterLoadBalancerF5Config {
    /// The load balancer's IP address.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    /// he preexisting partition to be used by the load balancer. T
    /// his partition is usually created for the admin cluster for example:
    /// 'my-f5-admin-partition'.
    #[builder(into)]
    #[serde(rename = "partition")]
    pub r#partition: Option<String>,
    /// The pool name. Only necessary, if using SNAT.
    #[builder(into)]
    #[serde(rename = "snatPool")]
    pub r#snat_pool: Option<String>,
}
