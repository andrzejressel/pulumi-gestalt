#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServerlessCacheReaderEndpoint {
    /// The DNS hostname of the cache node.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: String,
    /// The port number that the cache engine is listening on. Set as integer.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: i32,
}
