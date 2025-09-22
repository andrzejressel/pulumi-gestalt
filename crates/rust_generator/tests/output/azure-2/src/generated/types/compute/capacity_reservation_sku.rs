#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CapacityReservationSku {
    /// Specifies the number of instances to be reserved. It must be greater than or equal to `0` and not exceed the quota in the subscription.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: i32,
    /// Name of the sku, such as `Standard_F2`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
