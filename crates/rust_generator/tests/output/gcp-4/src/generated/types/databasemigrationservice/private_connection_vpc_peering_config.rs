#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PrivateConnectionVpcPeeringConfig {
    /// A free subnet for peering. (CIDR of /29)
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "subnet")]
    pub r#subnet: String,
    /// Fully qualified name of the VPC that Database Migration Service will peer to.
    /// Format: projects/{project}/global/{networks}/{name}
    #[builder(into)]
    #[serde(rename = "vpcName")]
    pub r#vpc_name: String,
}
