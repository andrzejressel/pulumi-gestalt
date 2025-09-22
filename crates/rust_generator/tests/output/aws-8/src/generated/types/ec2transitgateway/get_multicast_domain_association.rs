#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetMulticastDomainAssociation {
    /// The ID of the subnet associated with the transit gateway multicast domain.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
    /// The ID of the transit gateway attachment.
    #[builder(into)]
    #[serde(rename = "transitGatewayAttachmentId")]
    pub r#transit_gateway_attachment_id: String,
}
