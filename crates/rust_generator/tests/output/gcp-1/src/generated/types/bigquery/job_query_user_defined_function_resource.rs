#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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
