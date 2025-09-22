#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ThreatIntelligenceIndicatorExternalReference {
    /// The description of the external reference of the Threat Intelligence Indicator.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The list of hashes of the external reference of the Threat Intelligence Indicator.
    #[builder(into)]
    #[serde(rename = "hashes")]
    pub r#hashes: Option<std::collections::HashMap<String, String>>,
    /// The ID of the Sentinel Threat Intelligence Indicator.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The source name of the external reference of the Threat Intelligence Indicator.
    #[builder(into)]
    #[serde(rename = "sourceName")]
    pub r#source_name: Option<String>,
    /// The url of the external reference of the Threat Intelligence Indicator.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}
