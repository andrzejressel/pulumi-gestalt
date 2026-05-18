#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationOutputFormatConfigurationSerializerParquetSerDe {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "block_size_bytes",
                    &self.r#block_size_bytes,
                ),
                to_pulumi_object_field(
                    "compression",
                    &self.r#compression,
                ),
                to_pulumi_object_field(
                    "enable_dictionary_compression",
                    &self.r#enable_dictionary_compression,
                ),
                to_pulumi_object_field(
                    "max_padding_bytes",
                    &self.r#max_padding_bytes,
                ),
                to_pulumi_object_field(
                    "page_size_bytes",
                    &self.r#page_size_bytes,
                ),
                to_pulumi_object_field(
                    "writer_version",
                    &self.r#writer_version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationOutputFormatConfigurationSerializerParquetSerDe {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#block_size_bytes: {
                        let field_value = match fields_map.get("block_size_bytes") {
                            Some(value) => value,
                            None => bail!("Missing field 'block_size_bytes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#compression: {
                        let field_value = match fields_map.get("compression") {
                            Some(value) => value,
                            None => bail!("Missing field 'compression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_dictionary_compression: {
                        let field_value = match fields_map.get("enable_dictionary_compression") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_dictionary_compression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_padding_bytes: {
                        let field_value = match fields_map.get("max_padding_bytes") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_padding_bytes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#page_size_bytes: {
                        let field_value = match fields_map.get("page_size_bytes") {
                            Some(value) => value,
                            None => bail!("Missing field 'page_size_bytes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#writer_version: {
                        let field_value = match fields_map.get("writer_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'writer_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
