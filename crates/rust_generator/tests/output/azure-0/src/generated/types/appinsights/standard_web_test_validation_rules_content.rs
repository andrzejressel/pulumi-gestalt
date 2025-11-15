#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StandardWebTestValidationRulesContent {
    /// A string value containing the content to match on.
    #[builder(into)]
    #[serde(rename = "contentMatch")]
    pub r#content_match: String,
    /// Ignore the casing in the `content_match` value.
    #[builder(into)]
    #[serde(rename = "ignoreCase")]
    pub r#ignore_case: Option<bool>,
    /// If the content of `content_match` is found, pass the test. If set to `false`, the WebTest is failing if the content of `content_match` is found.
    #[builder(into)]
    #[serde(rename = "passIfTextFound")]
    pub r#pass_if_text_found: Option<bool>,
}
