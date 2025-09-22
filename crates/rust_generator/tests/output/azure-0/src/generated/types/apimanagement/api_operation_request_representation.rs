#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ApiOperationRequestRepresentation {
    /// The Content Type of this representation, such as `application/json`.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: String,
    /// One or more `example` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "examples")]
    pub r#examples: Option<Vec<super::super::types::apimanagement::ApiOperationRequestRepresentationExample>>,
    /// One or more `form_parameter` block as defined above.
    /// 
    /// > **NOTE:** This is Required when `content_type` is set to `application/x-www-form-urlencoded` or `multipart/form-data`.
    #[builder(into)]
    #[serde(rename = "formParameters")]
    pub r#form_parameters: Option<Vec<super::super::types::apimanagement::ApiOperationRequestRepresentationFormParameter>>,
    /// The ID of an API Management Schema which represents this Response.
    /// 
    /// > **NOTE:** This can only be specified when `content_type` is not set to `application/x-www-form-urlencoded` or `multipart/form-data`.
    #[builder(into)]
    #[serde(rename = "schemaId")]
    pub r#schema_id: Option<String>,
    /// The Type Name defined by the Schema.
    /// 
    /// > **NOTE:** This can only be specified when `content_type` is not set to `application/x-www-form-urlencoded` or `multipart/form-data`.
    #[builder(into)]
    #[serde(rename = "typeName")]
    pub r#type_name: Option<String>,
}
