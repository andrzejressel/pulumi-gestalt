#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TopicRuleElasticsearch {
    /// The endpoint of your Elasticsearch domain.
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: String,
    /// The unique identifier for the document you are storing.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// The Elasticsearch index where you want to store your data.
    #[builder(into)]
    #[serde(rename = "index")]
    pub r#index: String,
    /// The IAM role ARN that has access to Elasticsearch.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: String,
    /// The type of document you are storing.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
