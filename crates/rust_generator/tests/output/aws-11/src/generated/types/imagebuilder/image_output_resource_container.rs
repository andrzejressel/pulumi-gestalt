#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ImageOutputResourceContainer {
    /// Set of URIs for created containers.
    #[builder(into)]
    #[serde(rename = "imageUris")]
    pub r#image_uris: Option<Vec<String>>,
    /// Region of the container image.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
}
