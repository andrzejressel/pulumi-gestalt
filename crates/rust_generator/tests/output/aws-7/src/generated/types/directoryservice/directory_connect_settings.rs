#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DirectoryConnectSettings {
    #[builder(into)]
    #[serde(rename = "availabilityZones")]
    pub r#availability_zones: Option<Vec<String>>,
    /// The IP addresses of the AD Connector servers.
    #[builder(into)]
    #[serde(rename = "connectIps")]
    pub r#connect_ips: Option<Vec<String>>,
    /// The DNS IP addresses of the domain to connect to.
    #[builder(into)]
    #[serde(rename = "customerDnsIps")]
    pub r#customer_dns_ips: Vec<String>,
    /// The username corresponding to the password provided.
    #[builder(into)]
    #[serde(rename = "customerUsername")]
    pub r#customer_username: String,
    /// The identifiers of the subnets for the directory servers (2 subnets in 2 different AZs).
    #[builder(into)]
    #[serde(rename = "subnetIds")]
    pub r#subnet_ids: Vec<String>,
    /// The identifier of the VPC that the directory is in.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: String,
}
