#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for TransferJobTransferSpec {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "aws_s_3_data_source".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#aws_s_3_data_source,
                )
                .await,
            );
            map.insert(
                "azure_blob_storage_data_source".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#azure_blob_storage_data_source,
                )
                .await,
            );
            map.insert(
                "gcs_data_sink".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gcs_data_sink,
                )
                .await,
            );
            map.insert(
                "gcs_data_source".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gcs_data_source,
                )
                .await,
            );
            map.insert(
                "hdfs_data_source".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hdfs_data_source,
                )
                .await,
            );
            map.insert(
                "http_data_source".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#http_data_source,
                )
                .await,
            );
            map.insert(
                "object_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#object_conditions,
                )
                .await,
            );
            map.insert(
                "posix_data_sink".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#posix_data_sink,
                )
                .await,
            );
            map.insert(
                "posix_data_source".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#posix_data_source,
                )
                .await,
            );
            map.insert(
                "sink_agent_pool_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sink_agent_pool_name,
                )
                .await,
            );
            map.insert(
                "source_agent_pool_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_agent_pool_name,
                )
                .await,
            );
            map.insert(
                "transfer_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#transfer_options,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for TransferJobTransferSpec {
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
                    r#aws_s_3_data_source: {
                        let field_value = match fields_map.get("aws_s_3_data_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_s_3_data_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#azure_blob_storage_data_source: {
                        let field_value = match fields_map.get("azure_blob_storage_data_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'azure_blob_storage_data_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcs_data_sink: {
                        let field_value = match fields_map.get("gcs_data_sink") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcs_data_sink' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcs_data_source: {
                        let field_value = match fields_map.get("gcs_data_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcs_data_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hdfs_data_source: {
                        let field_value = match fields_map.get("hdfs_data_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'hdfs_data_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_data_source: {
                        let field_value = match fields_map.get("http_data_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_data_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#object_conditions: {
                        let field_value = match fields_map.get("object_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'object_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#posix_data_sink: {
                        let field_value = match fields_map.get("posix_data_sink") {
                            Some(value) => value,
                            None => bail!("Missing field 'posix_data_sink' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#posix_data_source: {
                        let field_value = match fields_map.get("posix_data_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'posix_data_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sink_agent_pool_name: {
                        let field_value = match fields_map.get("sink_agent_pool_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'sink_agent_pool_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_agent_pool_name: {
                        let field_value = match fields_map.get("source_agent_pool_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_agent_pool_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transfer_options: {
                        let field_value = match fields_map.get("transfer_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'transfer_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
