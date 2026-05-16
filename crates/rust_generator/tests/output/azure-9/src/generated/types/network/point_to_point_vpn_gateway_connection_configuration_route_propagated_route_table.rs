#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PointToPointVpnGatewayConnectionConfigurationRoutePropagatedRouteTable {
    /// The list of Virtual Hub Route Table resource id which the routes will be propagated to.
    #[builder(into)]
    #[serde(rename = "ids")]
    pub r#ids: Vec<String>,
    /// The list of labels to logically group Virtual Hub Route Tables which the routes will be propagated to.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Option<Vec<String>>,
}
