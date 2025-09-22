#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ReservationSpecificReservation {
    /// The number of resources that are allocated.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: i32,
    /// (Output)
    /// How many instances are in use.
    #[builder(into)]
    #[serde(rename = "inUseCount")]
    pub r#in_use_count: Option<i32>,
    /// The instance properties for the reservation.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "instanceProperties")]
    pub r#instance_properties: Box<super::super::types::compute::ReservationSpecificReservationInstanceProperties>,
}
