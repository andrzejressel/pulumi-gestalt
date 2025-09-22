#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkConnectionMonitorTestGroup {
    /// A list of destination endpoint names.
    #[builder(into)]
    #[serde(rename = "destinationEndpoints")]
    pub r#destination_endpoints: Vec<String>,
    /// Should the test group be enabled? Defaults to `true`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// The name of the test group for the Network Connection Monitor.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// A list of source endpoint names.
    #[builder(into)]
    #[serde(rename = "sourceEndpoints")]
    pub r#source_endpoints: Vec<String>,
    /// A list of test configuration names.
    #[builder(into)]
    #[serde(rename = "testConfigurationNames")]
    pub r#test_configuration_names: Vec<String>,
}
