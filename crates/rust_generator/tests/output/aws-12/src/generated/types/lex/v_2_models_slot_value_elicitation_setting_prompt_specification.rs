#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct V2ModelsSlotValueElicitationSettingPromptSpecification {
    #[builder(into)]
    #[serde(rename = "allowInterrupt")]
    pub r#allow_interrupt: Option<bool>,
    #[builder(into)]
    #[serde(rename = "maxRetries")]
    pub r#max_retries: i32,
    #[builder(into)]
    #[serde(rename = "messageGroups")]
    pub r#message_groups: Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationMessageGroup>>,
    #[builder(into)]
    #[serde(rename = "messageSelectionStrategy")]
    pub r#message_selection_strategy: Option<String>,
    #[builder(into)]
    #[serde(rename = "promptAttemptsSpecifications")]
    pub r#prompt_attempts_specifications: Option<Vec<super::super::types::lex::V2ModelsSlotValueElicitationSettingPromptSpecificationPromptAttemptsSpecification>>,
}
