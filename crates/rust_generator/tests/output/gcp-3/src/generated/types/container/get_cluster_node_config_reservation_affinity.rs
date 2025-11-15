#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetClusterNodeConfigReservationAffinity {
    /// Corresponds to the type of reservation consumption.
    #[builder(into)]
    #[serde(rename = "consumeReservationType")]
    pub r#consume_reservation_type: String,
    /// The label key of a reservation resource.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: String,
    /// The label values of the reservation resource.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Vec<String>,
}
