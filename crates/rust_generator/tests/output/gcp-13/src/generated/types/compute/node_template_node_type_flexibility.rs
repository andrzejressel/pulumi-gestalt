#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NodeTemplateNodeTypeFlexibility {
    /// Number of virtual CPUs to use.
    #[builder(into)]
    #[serde(rename = "cpus")]
    pub r#cpus: Option<String>,
    /// (Output)
    /// Use local SSD
    #[builder(into)]
    #[serde(rename = "localSsd")]
    pub r#local_ssd: Option<String>,
    /// Physical memory available to the node, defined in MB.
    #[builder(into)]
    #[serde(rename = "memory")]
    pub r#memory: Option<String>,
}
