#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DocumentationPartLocation {
    /// HTTP verb of a method. The default value is `*` for any method.
    #[builder(into)]
    #[serde(rename = "method")]
    pub r#method: Option<String>,
    /// Name of the targeted API entity.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// URL path of the target. The default value is `/` for the root resource.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// HTTP status code of a response. The default value is `*` for any status code.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<String>,
    /// Type of API entity to which the documentation content appliesE.g., `API`, `METHOD` or `REQUEST_BODY`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
