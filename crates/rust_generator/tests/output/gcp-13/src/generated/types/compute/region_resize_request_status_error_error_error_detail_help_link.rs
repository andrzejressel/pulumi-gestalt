#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionResizeRequestStatusErrorErrorErrorDetailHelpLink {
    /// An optional description of this resize-request.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// (Output)
    /// The URL of the link.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Option<String>,
}
