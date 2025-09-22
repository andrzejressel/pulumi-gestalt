#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AgentAgentPromptOverrideConfiguration {
    /// ARN of the Lambda function to use when parsing the raw foundation model output in parts of the agent sequence. If you specify this field, at least one of the `prompt_configurations` block must contain a `parser_mode` value that is set to `OVERRIDDEN`.
    #[builder(into)]
    #[serde(rename = "overrideLambda")]
    pub r#override_lambda: String,
    /// Configurations to override a prompt template in one part of an agent sequence. See `prompt_configurations` Block for details.
    #[builder(into)]
    #[serde(rename = "promptConfigurations")]
    pub r#prompt_configurations: Vec<super::super::types::bedrock::AgentAgentPromptOverrideConfigurationPromptConfiguration>,
}
