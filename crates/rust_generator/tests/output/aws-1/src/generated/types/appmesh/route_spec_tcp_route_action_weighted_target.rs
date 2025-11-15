#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RouteSpecTcpRouteActionWeightedTarget {
    /// The targeted port of the weighted object.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// Virtual node to associate with the weighted target. Must be between 1 and 255 characters in length.
    #[builder(into)]
    #[serde(rename = "virtualNode")]
    pub r#virtual_node: String,
    /// Relative weight of the weighted target. An integer between 0 and 100.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: i32,
}
