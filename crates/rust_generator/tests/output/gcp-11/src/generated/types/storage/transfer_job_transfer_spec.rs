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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "awsS3DataSource",
                    &self.r#aws_s_3_data_source,
                ),
                to_pulumi_object_field(
                    "azureBlobStorageDataSource",
                    &self.r#azure_blob_storage_data_source,
                ),
                to_pulumi_object_field(
                    "gcsDataSink",
                    &self.r#gcs_data_sink,
                ),
                to_pulumi_object_field(
                    "gcsDataSource",
                    &self.r#gcs_data_source,
                ),
                to_pulumi_object_field(
                    "hdfsDataSource",
                    &self.r#hdfs_data_source,
                ),
                to_pulumi_object_field(
                    "httpDataSource",
                    &self.r#http_data_source,
                ),
                to_pulumi_object_field(
                    "objectConditions",
                    &self.r#object_conditions,
                ),
                to_pulumi_object_field(
                    "posixDataSink",
                    &self.r#posix_data_sink,
                ),
                to_pulumi_object_field(
                    "posixDataSource",
                    &self.r#posix_data_source,
                ),
                to_pulumi_object_field(
                    "sinkAgentPoolName",
                    &self.r#sink_agent_pool_name,
                ),
                to_pulumi_object_field(
                    "sourceAgentPoolName",
                    &self.r#source_agent_pool_name,
                ),
                to_pulumi_object_field(
                    "transferOptions",
                    &self.r#transfer_options,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("awsS3DataSource") {
                            Some(value) => value,
                            None => bail!("Missing field 'awsS3DataSource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#azure_blob_storage_data_source: {
                        let field_value = match fields_map.get("azureBlobStorageDataSource") {
                            Some(value) => value,
                            None => bail!("Missing field 'azureBlobStorageDataSource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcs_data_sink: {
                        let field_value = match fields_map.get("gcsDataSink") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcsDataSink' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#gcs_data_source: {
                        let field_value = match fields_map.get("gcsDataSource") {
                            Some(value) => value,
                            None => bail!("Missing field 'gcsDataSource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hdfs_data_source: {
                        let field_value = match fields_map.get("hdfsDataSource") {
                            Some(value) => value,
                            None => bail!("Missing field 'hdfsDataSource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_data_source: {
                        let field_value = match fields_map.get("httpDataSource") {
                            Some(value) => value,
                            None => bail!("Missing field 'httpDataSource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#object_conditions: {
                        let field_value = match fields_map.get("objectConditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'objectConditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#posix_data_sink: {
                        let field_value = match fields_map.get("posixDataSink") {
                            Some(value) => value,
                            None => bail!("Missing field 'posixDataSink' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#posix_data_source: {
                        let field_value = match fields_map.get("posixDataSource") {
                            Some(value) => value,
                            None => bail!("Missing field 'posixDataSource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sink_agent_pool_name: {
                        let field_value = match fields_map.get("sinkAgentPoolName") {
                            Some(value) => value,
                            None => bail!("Missing field 'sinkAgentPoolName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_agent_pool_name: {
                        let field_value = match fields_map.get("sourceAgentPoolName") {
                            Some(value) => value,
                            None => bail!("Missing field 'sourceAgentPoolName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#transfer_options: {
                        let field_value = match fields_map.get("transferOptions") {
                            Some(value) => value,
                            None => bail!("Missing field 'transferOptions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
