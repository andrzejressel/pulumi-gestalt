#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RecordGeoproximityRoutingPolicyCoordinate {
    #[builder(into)]
    #[serde(rename = "latitude")]
    pub r#latitude: Box<String>,
    #[builder(into)]
    #[serde(rename = "longitude")]
    pub r#longitude: Box<String>,
}
