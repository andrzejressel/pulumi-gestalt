#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceListenerEndpoint {
    /// Specifies the DNS address of the DB instance.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    /// Specifies the ID that Amazon Route 53 assigns when you create a hosted zone.
    #[builder(into)]
    #[serde(rename = "hostedZoneId")]
    pub r#hosted_zone_id: Option<String>,
    /// The port on which the DB accepts connections.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
}
