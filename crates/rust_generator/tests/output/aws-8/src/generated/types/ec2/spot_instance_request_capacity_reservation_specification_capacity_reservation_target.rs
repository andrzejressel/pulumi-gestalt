#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SpotInstanceRequestCapacityReservationSpecificationCapacityReservationTarget {
    /// ID of the Capacity Reservation in which to run the instance.
    #[builder(into)]
    #[serde(rename = "capacityReservationId")]
    pub r#capacity_reservation_id: Option<String>,
    /// ARN of the Capacity Reservation resource group in which to run the instance.
    #[builder(into)]
    #[serde(rename = "capacityReservationResourceGroupArn")]
    pub r#capacity_reservation_resource_group_arn: Option<String>,
}
