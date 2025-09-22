#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationOutputFormatConfigurationSerializerParquetSerDe {
    /// The Hadoop Distributed File System (HDFS) block size. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is 256 MiB and the minimum is 64 MiB. Kinesis Data Firehose uses this value for padding calculations.
    #[builder(into)]
    #[serde(rename = "blockSizeBytes")]
    pub r#block_size_bytes: Option<i32>,
    /// The compression code to use over data blocks. The possible values are `UNCOMPRESSED`, `SNAPPY`, and `GZIP`, with the default being `SNAPPY`. Use `SNAPPY` for higher decompression speed. Use `GZIP` if the compression ratio is more important than speed.
    #[builder(into)]
    #[serde(rename = "compression")]
    pub r#compression: Option<String>,
    /// Indicates whether to enable dictionary compression.
    #[builder(into)]
    #[serde(rename = "enableDictionaryCompression")]
    pub r#enable_dictionary_compression: Option<bool>,
    /// The maximum amount of padding to apply. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is `0`.
    #[builder(into)]
    #[serde(rename = "maxPaddingBytes")]
    pub r#max_padding_bytes: Option<i32>,
    /// The Parquet page size. Column chunks are divided into pages. A page is conceptually an indivisible unit (in terms of compression and encoding). The minimum value is 64 KiB and the default is 1 MiB.
    #[builder(into)]
    #[serde(rename = "pageSizeBytes")]
    pub r#page_size_bytes: Option<i32>,
    /// Indicates the version of row format to output. The possible values are `V1` and `V2`. The default is `V1`.
    #[builder(into)]
    #[serde(rename = "writerVersion")]
    pub r#writer_version: Option<String>,
}
