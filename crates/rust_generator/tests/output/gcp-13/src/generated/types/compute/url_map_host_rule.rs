#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UrlMapHostRule {
    /// An optional description of this resource. Provide this property when you create
    /// the resource.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The list of host patterns to match. They must be valid hostnames, except * will
    /// match any string of ([a-z0-9-.]*). In that case, * must be the first character
    /// and must be followed in the pattern by either - or ..
    #[builder(into)]
    #[serde(rename = "hosts")]
    pub r#hosts: Vec<String>,
    /// The name of the PathMatcher to use to match the path portion of the URL if the
    /// hostRule matches the URL's host portion.
    #[builder(into)]
    #[serde(rename = "pathMatcher")]
    pub r#path_matcher: String,
}
