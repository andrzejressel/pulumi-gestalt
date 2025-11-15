#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetElasticPoolSkus {
    /// The scale up/out capacity, representing server's compute units.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: i32,
    /// The `family` of hardware.
    #[builder(into)]
    #[serde(rename = "family")]
    pub r#family: String,
    /// The name of the elastic pool.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The tier of the particular SKU.
    #[builder(into)]
    #[serde(rename = "tier")]
    pub r#tier: String,
}
