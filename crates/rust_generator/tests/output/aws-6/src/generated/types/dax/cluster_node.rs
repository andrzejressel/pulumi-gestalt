#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNode {
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    #[builder(into)]
    #[serde(rename = "availabilityZone")]
    pub r#availability_zone: Option<String>,
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The port used by the configuration endpoint
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
}
