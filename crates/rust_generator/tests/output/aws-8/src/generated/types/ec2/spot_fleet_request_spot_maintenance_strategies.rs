#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpotFleetRequestSpotMaintenanceStrategies {
    /// Nested argument containing the capacity rebalance for your fleet request. Defined below.
    #[builder(into)]
    #[serde(rename = "capacityRebalance")]
    pub r#capacity_rebalance: Option<Box<super::super::types::ec2::SpotFleetRequestSpotMaintenanceStrategiesCapacityRebalance>>,
}
