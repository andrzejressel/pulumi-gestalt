#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GrpcRouteRuleActionDestination {
    /// The URL of a BackendService to route traffic to.
    #[builder(into, default)]
    #[serde(rename = "serviceName")]
    pub r#service_name: Box<Option<String>>,
    /// Specifies the proportion of requests forwarded to the backend referenced by the serviceName field.
    #[builder(into, default)]
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<i32>>,
}
