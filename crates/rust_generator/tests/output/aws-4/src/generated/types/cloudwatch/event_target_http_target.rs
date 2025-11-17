#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EventTargetHttpTarget {
    /// Enables you to specify HTTP headers to add to the request.
    #[builder(into)]
    #[serde(rename = "headerParameters")]
    pub r#header_parameters: Option<std::collections::HashMap<String, String>>,
    /// The list of values that correspond sequentially to any path variables in your endpoint ARN (for example `arn:aws:execute-api:us-east-1:123456:myapi/*/POST/pets/*`).
    #[builder(into)]
    #[serde(rename = "pathParameterValues")]
    pub r#path_parameter_values: Option<Vec<String>>,
    /// Represents keys/values of query string parameters that are appended to the invoked endpoint.
    #[builder(into)]
    #[serde(rename = "queryStringParameters")]
    pub r#query_string_parameters: Option<std::collections::HashMap<String, String>>,
}
