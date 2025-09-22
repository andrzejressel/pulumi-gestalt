#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TransferJobTransferSpec {
    /// An AWS S3 data source. Structure documented below.
    #[builder(into)]
    #[serde(rename = "awsS3DataSource")]
    pub r#aws_s_3_data_source: Option<Box<super::super::types::storage::TransferJobTransferSpecAwsS3DataSource>>,
    /// An Azure Blob Storage data source. Structure documented below.
    #[builder(into)]
    #[serde(rename = "azureBlobStorageDataSource")]
    pub r#azure_blob_storage_data_source: Option<Box<super::super::types::storage::TransferJobTransferSpecAzureBlobStorageDataSource>>,
    /// A Google Cloud Storage data sink. Structure documented below.
    #[builder(into)]
    #[serde(rename = "gcsDataSink")]
    pub r#gcs_data_sink: Option<Box<super::super::types::storage::TransferJobTransferSpecGcsDataSink>>,
    /// A Google Cloud Storage data source. Structure documented below.
    #[builder(into)]
    #[serde(rename = "gcsDataSource")]
    pub r#gcs_data_source: Option<Box<super::super::types::storage::TransferJobTransferSpecGcsDataSource>>,
    /// An HDFS data source. Structure documented below.
    #[builder(into)]
    #[serde(rename = "hdfsDataSource")]
    pub r#hdfs_data_source: Option<Box<super::super::types::storage::TransferJobTransferSpecHdfsDataSource>>,
    /// A HTTP URL data source. Structure documented below.
    #[builder(into)]
    #[serde(rename = "httpDataSource")]
    pub r#http_data_source: Option<Box<super::super::types::storage::TransferJobTransferSpecHttpDataSource>>,
    /// Only objects that satisfy these object conditions are included in the set of data source and data sink objects. Object conditions based on objects' `last_modification_time` do not exclude objects in a data sink. Structure documented below.
    #[builder(into)]
    #[serde(rename = "objectConditions")]
    pub r#object_conditions: Option<Box<super::super::types::storage::TransferJobTransferSpecObjectConditions>>,
    /// A POSIX data sink. Structure documented below.
    #[builder(into)]
    #[serde(rename = "posixDataSink")]
    pub r#posix_data_sink: Option<Box<super::super::types::storage::TransferJobTransferSpecPosixDataSink>>,
    /// A POSIX filesystem data source. Structure documented below.
    #[builder(into)]
    #[serde(rename = "posixDataSource")]
    pub r#posix_data_source: Option<Box<super::super::types::storage::TransferJobTransferSpecPosixDataSource>>,
    /// Specifies the agent pool name associated with the posix data sink. When unspecified, the default name is used.
    #[builder(into)]
    #[serde(rename = "sinkAgentPoolName")]
    pub r#sink_agent_pool_name: Option<String>,
    /// Specifies the agent pool name associated with the posix data source. When unspecified, the default name is used.
    #[builder(into)]
    #[serde(rename = "sourceAgentPoolName")]
    pub r#source_agent_pool_name: Option<String>,
    /// Characteristics of how to treat files from datasource and sink during job. If the option `delete_objects_unique_in_sink` is true, object conditions based on objects' `last_modification_time` are ignored and do not exclude objects in a data source or a data sink. Structure documented below.
    #[builder(into)]
    #[serde(rename = "transferOptions")]
    pub r#transfer_options: Option<Box<super::super::types::storage::TransferJobTransferSpecTransferOptions>>,
}
