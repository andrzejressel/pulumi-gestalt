#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MetadataCategory {
    /// Specifies a list of domains for the solution content item.
    #[builder(into)]
    #[serde(rename = "domains")]
    pub r#domains: Option<Vec<String>>,
    /// Specifies a list of industry verticals for the solution content item.
    #[builder(into)]
    #[serde(rename = "verticals")]
    pub r#verticals: Option<Vec<String>>,
}
