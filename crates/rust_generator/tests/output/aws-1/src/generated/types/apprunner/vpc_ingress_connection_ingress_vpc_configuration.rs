#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VpcIngressConnectionIngressVpcConfiguration {
    /// The ID of the VPC endpoint that your App Runner service connects to.
    #[builder(into)]
    #[serde(rename = "vpcEndpointId")]
    pub r#vpc_endpoint_id: Option<String>,
    /// The ID of the VPC that is used for the VPC endpoint.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Option<String>,
}
