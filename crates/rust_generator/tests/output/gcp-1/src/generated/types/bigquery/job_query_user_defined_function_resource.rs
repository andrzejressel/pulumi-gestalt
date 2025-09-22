#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct JobQueryUserDefinedFunctionResource {
    /// An inline resource that contains code for a user-defined function (UDF).
    /// Providing a inline code resource is equivalent to providing a URI for a file containing the same code.
    #[builder(into)]
    #[serde(rename = "inlineCode")]
    pub r#inline_code: Option<String>,
    /// A code resource to load from a Google Cloud Storage URI (gs://bucket/path).
    #[builder(into)]
    #[serde(rename = "resourceUri")]
    pub r#resource_uri: Option<String>,
}
