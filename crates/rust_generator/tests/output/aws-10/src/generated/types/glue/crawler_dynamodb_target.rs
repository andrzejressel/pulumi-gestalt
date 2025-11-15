#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CrawlerDynamodbTarget {
    /// The name of the DynamoDB table to crawl.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: String,
    /// Indicates whether to scan all the records, or to sample rows from the table. Scanning all the records can take a long time when the table is not a high throughput table.  defaults to `true`.
    #[builder(into)]
    #[serde(rename = "scanAll")]
    pub r#scan_all: Option<bool>,
    /// The percentage of the configured read capacity units to use by the AWS Glue crawler. The valid values are null or a value between 0.1 to 1.5.
    #[builder(into)]
    #[serde(rename = "scanRate")]
    pub r#scan_rate: Option<f64>,
}
