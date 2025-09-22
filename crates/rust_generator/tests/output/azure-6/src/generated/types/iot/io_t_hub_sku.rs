#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IoTHubSku {
    /// The number of provisioned IoT Hub units.
    /// 
    /// > **NOTE:** Only one IotHub can be on the `Free` tier per subscription.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: i32,
    /// The name of the sku. Possible values are `B1`, `B2`, `B3`, `F1`, `S1`, `S2`, and `S3`.
    /// 
    /// > **NOTE:** The `F1` sku is on `Free` tier.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
