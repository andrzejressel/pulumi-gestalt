#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataQualityJobDefinitionDataQualityJobInputEndpointInput {
    /// An endpoint in customer's account which has `data_capture_config` enabled.
    #[builder(into)]
    #[serde(rename = "endpointName")]
    pub r#endpoint_name: String,
    /// Path to the filesystem where the endpoint data is available to the container. Defaults to `/opt/ml/processing/input`.
    #[builder(into)]
    #[serde(rename = "localPath")]
    pub r#local_path: Option<String>,
    /// Whether input data distributed in Amazon S3 is fully replicated or sharded by an S3 key. Defaults to `FullyReplicated`. Valid values are `FullyReplicated` or `ShardedByS3Key`
    #[builder(into)]
    #[serde(rename = "s3DataDistributionType")]
    pub r#s_3_data_distribution_type: Option<String>,
    /// Whether the `Pipe` or `File` is used as the input mode for transferring data for the monitoring job. `Pipe` mode is recommended for large datasets. `File` mode is useful for small files that fit in memory. Defaults to `File`.  Valid values are `Pipe` or `File`
    #[builder(into)]
    #[serde(rename = "s3InputMode")]
    pub r#s_3_input_mode: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataQualityJobDefinitionDataQualityJobInputEndpointInput {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("endpoint_name".to_string(), self.r#endpoint_name.to_pulumi_value().await);
            map.insert("local_path".to_string(), self.r#local_path.to_pulumi_value().await);
            map.insert("s_3_data_distribution_type".to_string(), self.r#s_3_data_distribution_type.to_pulumi_value().await);
            map.insert("s_3_input_mode".to_string(), self.r#s_3_input_mode.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataQualityJobDefinitionDataQualityJobInputEndpointInput {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#endpoint_name: {
                        let field_value = match fields_map.get("endpoint_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'endpoint_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#local_path: {
                        let field_value = match fields_map.get("local_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_data_distribution_type: {
                        let field_value = match fields_map.get("s_3_data_distribution_type") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_data_distribution_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_input_mode: {
                        let field_value = match fields_map.get("s_3_input_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_input_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
