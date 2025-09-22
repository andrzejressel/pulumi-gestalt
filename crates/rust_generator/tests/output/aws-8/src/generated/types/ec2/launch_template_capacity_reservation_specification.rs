#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LaunchTemplateCapacityReservationSpecification {
    /// Indicates the instance's Capacity Reservation preferences. Can be `open` or `none`. (Default `none`).
    #[builder(into)]
    #[serde(rename = "capacityReservationPreference")]
    pub r#capacity_reservation_preference: Option<String>,
    /// Used to target a specific Capacity Reservation:
    #[builder(into)]
    #[serde(rename = "capacityReservationTarget")]
    pub r#capacity_reservation_target: Box<Option<super::super::types::ec2::LaunchTemplateCapacityReservationSpecificationCapacityReservationTarget>>,
}
