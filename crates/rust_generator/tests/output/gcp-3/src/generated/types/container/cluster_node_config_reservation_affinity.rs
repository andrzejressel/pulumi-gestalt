#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterNodeConfigReservationAffinity {
    /// The type of reservation consumption
    /// Accepted values are:
    /// 
    /// * `"UNSPECIFIED"`: Default value. This should not be used.
    /// * `"NO_RESERVATION"`: Do not consume from any reserved capacity.
    /// * `"ANY_RESERVATION"`: Consume any reservation available.
    /// * `"SPECIFIC_RESERVATION"`: Must consume from a specific reservation. Must specify key value fields for specifying the reservations.
    #[builder(into)]
    #[serde(rename = "consumeReservationType")]
    pub r#consume_reservation_type: String,
    /// The label key of a reservation resource. To target a SPECIFIC_RESERVATION by name, specify "compute.googleapis.com/reservation-name" as the key and specify the name of your reservation as its value.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Option<String>,
    /// The list of label values of reservation resources. For example: the name of the specific reservation when using a key of "compute.googleapis.com/reservation-name"
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Option<Vec<String>>,
}
