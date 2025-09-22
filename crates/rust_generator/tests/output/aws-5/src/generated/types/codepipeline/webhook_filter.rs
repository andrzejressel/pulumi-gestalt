#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebhookFilter {
    /// The [JSON path](https://github.com/json-path/JsonPath) to filter on.
    #[builder(into)]
    #[serde(rename = "jsonPath")]
    pub r#json_path: String,
    /// The value to match on (e.g., `refs/heads/{Branch}`). See [AWS docs](https://docs.aws.amazon.com/codepipeline/latest/APIReference/API_WebhookFilterRule.html) for details.
    #[builder(into)]
    #[serde(rename = "matchEquals")]
    pub r#match_equals: String,
}
