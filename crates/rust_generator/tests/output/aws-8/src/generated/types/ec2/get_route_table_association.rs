#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetRouteTableAssociation {
    /// ID of an Internet Gateway or Virtual Private Gateway which is connected to the Route Table (not exported if not passed as a parameter).
    #[builder(into)]
    #[serde(rename = "gatewayId")]
    pub r#gateway_id: String,
    /// Whether the association is due to the main route table.
    #[builder(into)]
    #[serde(rename = "main")]
    pub r#main: bool,
    /// Association ID.
    #[builder(into)]
    #[serde(rename = "routeTableAssociationId")]
    pub r#route_table_association_id: String,
    /// ID of the specific Route Table to retrieve.
    #[builder(into)]
    #[serde(rename = "routeTableId")]
    pub r#route_table_id: String,
    /// ID of a Subnet which is connected to the Route Table (not exported if not passed as a parameter).
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}
