#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRulesetsRulesetRuleActionParametersResponse {
    /// Body content to include in the response.
    #[builder(into)]
    #[serde(rename = "content")]
    pub r#content: Option<String>,
    /// HTTP content type to send in the response.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: Option<String>,
    /// HTTP status code to send in the response.
    #[builder(into)]
    #[serde(rename = "statusCode")]
    pub r#status_code: Option<i32>,
}
