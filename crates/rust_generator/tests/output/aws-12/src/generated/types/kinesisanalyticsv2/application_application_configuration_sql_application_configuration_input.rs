#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationInput {
    #[builder(into)]
    #[serde(rename = "inAppStreamNames")]
    pub r#in_app_stream_names: Option<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "inputId")]
    pub r#input_id: Option<String>,
    /// Describes the number of in-application streams to create.
    #[builder(into)]
    #[serde(rename = "inputParallelism")]
    pub r#input_parallelism: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputParallelism>>,
    /// The input processing configuration for the input.
    /// An input processor transforms records as they are received from the stream, before the application's SQL code executes.
    #[builder(into)]
    #[serde(rename = "inputProcessingConfiguration")]
    pub r#input_processing_configuration: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputProcessingConfiguration>>,
    /// Describes the format of the data in the streaming source, and how each data element maps to corresponding columns in the in-application stream that is being created.
    #[builder(into)]
    #[serde(rename = "inputSchema")]
    pub r#input_schema: Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputSchema>,
    /// The point at which the application starts processing records from the streaming source.
    #[builder(into)]
    #[serde(rename = "inputStartingPositionConfigurations")]
    pub r#input_starting_position_configurations: Option<Vec<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputStartingPositionConfiguration>>,
    /// If the streaming source is a Kinesis Data Firehose delivery stream, identifies the delivery stream's ARN.
    #[builder(into)]
    #[serde(rename = "kinesisFirehoseInput")]
    pub r#kinesis_firehose_input: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputKinesisFirehoseInput>>,
    /// If the streaming source is a Kinesis data stream, identifies the stream's Amazon Resource Name (ARN).
    #[builder(into)]
    #[serde(rename = "kinesisStreamsInput")]
    pub r#kinesis_streams_input: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputKinesisStreamsInput>>,
    /// The name prefix to use when creating an in-application stream.
    #[builder(into)]
    #[serde(rename = "namePrefix")]
    pub r#name_prefix: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationApplicationConfigurationSqlApplicationConfigurationInput {
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
                "in_app_stream_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#in_app_stream_names,
                )
                .await,
            );
            map.insert(
                "input_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_id,
                )
                .await,
            );
            map.insert(
                "input_parallelism".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_parallelism,
                )
                .await,
            );
            map.insert(
                "input_processing_configuration".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_processing_configuration,
                )
                .await,
            );
            map.insert(
                "input_schema".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_schema,
                )
                .await,
            );
            map.insert(
                "input_starting_position_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input_starting_position_configurations,
                )
                .await,
            );
            map.insert(
                "kinesis_firehose_input".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kinesis_firehose_input,
                )
                .await,
            );
            map.insert(
                "kinesis_streams_input".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kinesis_streams_input,
                )
                .await,
            );
            map.insert(
                "name_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name_prefix,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationApplicationConfigurationSqlApplicationConfigurationInput {
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
                    r#in_app_stream_names: {
                        let field_value = match fields_map.get("in_app_stream_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'in_app_stream_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_id: {
                        let field_value = match fields_map.get("input_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_parallelism: {
                        let field_value = match fields_map.get("input_parallelism") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_parallelism' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_processing_configuration: {
                        let field_value = match fields_map.get("input_processing_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_processing_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_schema: {
                        let field_value = match fields_map.get("input_schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_starting_position_configurations: {
                        let field_value = match fields_map.get("input_starting_position_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_starting_position_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_firehose_input: {
                        let field_value = match fields_map.get("kinesis_firehose_input") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_firehose_input' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_streams_input: {
                        let field_value = match fields_map.get("kinesis_streams_input") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_streams_input' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name_prefix: {
                        let field_value = match fields_map.get("name_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'name_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
