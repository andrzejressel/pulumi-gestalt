#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AgentAgentActionGroupFunctionSchemaMemberFunctionsFunctionParameter {
    /// Description of the parameter. Helps the foundation model determine how to elicit the parameters from the user.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Name of the parameter.
    /// 
    /// **Note:** The argument name `map_block_key` may seem out of context, but is necessary for backward compatibility reasons in the provider.
    #[builder(into)]
    #[serde(rename = "mapBlockKey")]
    pub r#map_block_key: String,
    /// Whether the parameter is required for the agent to complete the function for action group invocation.
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: Option<bool>,
    /// Data type of the parameter. Valid values: `string`, `number`, `integer`, `boolean`, `array`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
