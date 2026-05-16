#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
