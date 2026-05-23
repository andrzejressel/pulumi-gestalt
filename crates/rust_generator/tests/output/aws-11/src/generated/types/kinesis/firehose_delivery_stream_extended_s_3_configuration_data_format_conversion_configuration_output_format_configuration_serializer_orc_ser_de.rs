#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationOutputFormatConfigurationSerializerOrcSerDe {
    /// The Hadoop Distributed File System (HDFS) block size. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is 256 MiB and the minimum is 64 MiB. Kinesis Data Firehose uses this value for padding calculations.
    #[builder(into)]
    pub r#block_size_bytes: Option<i32>,
    /// A list of column names for which you want Kinesis Data Firehose to create bloom filters.
    #[builder(into)]
    pub r#bloom_filter_columns: Option<Vec<String>>,
    /// The Bloom filter false positive probability (FPP). The lower the FPP, the bigger the Bloom filter. The default value is `0.05`, the minimum is `0`, and the maximum is `1`.
    #[builder(into)]
    pub r#bloom_filter_false_positive_probability: Option<f64>,
    /// The compression code to use over data blocks. The default is `SNAPPY`.
    #[builder(into)]
    pub r#compression: Option<String>,
    /// A float that represents the fraction of the total number of non-null rows. To turn off dictionary encoding, set this fraction to a number that is less than the number of distinct keys in a dictionary. To always use dictionary encoding, set this threshold to `1`.
    #[builder(into)]
    pub r#dictionary_key_threshold: Option<f64>,
    /// Set this to `true` to indicate that you want stripes to be padded to the HDFS block boundaries. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is `false`.
    #[builder(into)]
    pub r#enable_padding: Option<bool>,
    /// The version of the file to write. The possible values are `V0_11` and `V0_12`. The default is `V0_12`.
    #[builder(into)]
    pub r#format_version: Option<String>,
    /// A float between 0 and 1 that defines the tolerance for block padding as a decimal fraction of stripe size. The default value is `0.05`, which means 5 percent of stripe size. For the default values of 64 MiB ORC stripes and 256 MiB HDFS blocks, the default block padding tolerance of 5 percent reserves a maximum of 3.2 MiB for padding within the 256 MiB block. In such a case, if the available size within the block is more than 3.2 MiB, a new, smaller stripe is inserted to fit within that space. This ensures that no stripe crosses block boundaries and causes remote reads within a node-local task. Kinesis Data Firehose ignores this parameter when `enable_padding` is `false`.
    #[builder(into)]
    pub r#padding_tolerance: Option<f64>,
    /// The number of rows between index entries. The default is `10000` and the minimum is `1000`.
    #[builder(into)]
    pub r#row_index_stride: Option<i32>,
    /// The number of bytes in each stripe. The default is 64 MiB and the minimum is 8 MiB.
    #[builder(into)]
    pub r#stripe_size_bytes: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationOutputFormatConfigurationSerializerOrcSerDe {
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
                    "blockSizeBytes",
                    &self.r#block_size_bytes,
                ),
                to_pulumi_object_field(
                    "bloomFilterColumns",
                    &self.r#bloom_filter_columns,
                ),
                to_pulumi_object_field(
                    "bloomFilterFalsePositiveProbability",
                    &self.r#bloom_filter_false_positive_probability,
                ),
                to_pulumi_object_field(
                    "compression",
                    &self.r#compression,
                ),
                to_pulumi_object_field(
                    "dictionaryKeyThreshold",
                    &self.r#dictionary_key_threshold,
                ),
                to_pulumi_object_field(
                    "enablePadding",
                    &self.r#enable_padding,
                ),
                to_pulumi_object_field(
                    "formatVersion",
                    &self.r#format_version,
                ),
                to_pulumi_object_field(
                    "paddingTolerance",
                    &self.r#padding_tolerance,
                ),
                to_pulumi_object_field(
                    "rowIndexStride",
                    &self.r#row_index_stride,
                ),
                to_pulumi_object_field(
                    "stripeSizeBytes",
                    &self.r#stripe_size_bytes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationOutputFormatConfigurationSerializerOrcSerDe {
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
                        let field_value = match fields_map.get("blockSizeBytes") {
                            Some(value) => value,
                            None => bail!("Missing field 'blockSizeBytes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bloom_filter_columns: {
                        let field_value = match fields_map.get("bloomFilterColumns") {
                            Some(value) => value,
                            None => bail!("Missing field 'bloomFilterColumns' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bloom_filter_false_positive_probability: {
                        let field_value = match fields_map.get("bloomFilterFalsePositiveProbability") {
                            Some(value) => value,
                            None => bail!("Missing field 'bloomFilterFalsePositiveProbability' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#dictionary_key_threshold: {
                        let field_value = match fields_map.get("dictionaryKeyThreshold") {
                            Some(value) => value,
                            None => bail!("Missing field 'dictionaryKeyThreshold' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_padding: {
                        let field_value = match fields_map.get("enablePadding") {
                            Some(value) => value,
                            None => bail!("Missing field 'enablePadding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#format_version: {
                        let field_value = match fields_map.get("formatVersion") {
                            Some(value) => value,
                            None => bail!("Missing field 'formatVersion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#padding_tolerance: {
                        let field_value = match fields_map.get("paddingTolerance") {
                            Some(value) => value,
                            None => bail!("Missing field 'paddingTolerance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#row_index_stride: {
                        let field_value = match fields_map.get("rowIndexStride") {
                            Some(value) => value,
                            None => bail!("Missing field 'rowIndexStride' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stripe_size_bytes: {
                        let field_value = match fields_map.get("stripeSizeBytes") {
                            Some(value) => value,
                            None => bail!("Missing field 'stripeSizeBytes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
