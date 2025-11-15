#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct IngestionDestinationDestinationConfigurationAuditLogDestination {
    /// Contains information about an Amazon Data Firehose delivery stream.
    #[builder(into)]
    #[serde(rename = "firehoseStream")]
    pub r#firehose_stream: Option<Box<super::super::types::appfabric::IngestionDestinationDestinationConfigurationAuditLogDestinationFirehoseStream>>,
    /// Contains information about an Amazon S3 bucket.
    #[builder(into)]
    #[serde(rename = "s3Bucket")]
    pub r#s_3_bucket: Option<Box<super::super::types::appfabric::IngestionDestinationDestinationConfigurationAuditLogDestinationS3Bucket>>,
}
