#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TrafficMirrorFilterRuleDestinationPortRange {
    /// Starting port of the range
    #[builder(into)]
    #[serde(rename = "fromPort")]
    pub r#from_port: Option<i32>,
    /// Ending port of the range
    #[builder(into)]
    #[serde(rename = "toPort")]
    pub r#to_port: Option<i32>,
}
