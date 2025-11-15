#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApiOperationRequest {
    /// A description of the HTTP Request, which may include HTML tags.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// One or more `header` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<Vec<super::super::types::apimanagement::ApiOperationRequestHeader>>,
    /// One or more `query_parameter` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "queryParameters")]
    pub r#query_parameters: Option<Vec<super::super::types::apimanagement::ApiOperationRequestQueryParameter>>,
    /// One or more `representation` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "representations")]
    pub r#representations: Option<Vec<super::super::types::apimanagement::ApiOperationRequestRepresentation>>,
}
