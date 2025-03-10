#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FleetSpotOptionsMaintenanceStrategiesCapacityRebalance {
    /// The replacement strategy to use. Only available for fleets of `type` set to `maintain`. Valid values: `launch`.
    #[builder(into, default)]
    #[serde(rename = "replacementStrategy")]
    pub r#replacement_strategy: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "terminationDelay")]
    pub r#termination_delay: Box<Option<i32>>,
}
