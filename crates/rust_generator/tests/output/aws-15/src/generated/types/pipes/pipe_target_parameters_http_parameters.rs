#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PipeTargetParametersHttpParameters {
    #[builder(into)]
    #[serde(rename = "headerParameters")]
    pub r#header_parameters: Option<std::collections::HashMap<String, String>>,
    #[builder(into)]
    #[serde(rename = "pathParameterValues")]
    pub r#path_parameter_values: Option<String>,
    #[builder(into)]
    #[serde(rename = "queryStringParameters")]
    pub r#query_string_parameters: Option<std::collections::HashMap<String, String>>,
}
