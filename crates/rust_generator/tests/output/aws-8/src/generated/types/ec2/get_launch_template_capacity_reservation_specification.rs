#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLaunchTemplateCapacityReservationSpecification {
    #[builder(into)]
    #[serde(rename = "capacityReservationPreference")]
    pub r#capacity_reservation_preference: String,
    #[builder(into)]
    #[serde(rename = "capacityReservationTargets")]
    pub r#capacity_reservation_targets: Vec<super::super::types::ec2::GetLaunchTemplateCapacityReservationSpecificationCapacityReservationTarget>,
}
