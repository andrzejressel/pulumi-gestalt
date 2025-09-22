#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceReservationAffinity {
    /// The type of Compute Reservation.
    /// Possible values are: `NO_RESERVATION`, `ANY_RESERVATION`, `SPECIFIC_RESERVATION`.
    #[builder(into)]
    #[serde(rename = "consumeReservationType")]
    pub r#consume_reservation_type: String,
    /// Corresponds to the label key of reservation resource.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// Corresponds to the label values of reservation resource.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Option<Vec<String>>,
}
