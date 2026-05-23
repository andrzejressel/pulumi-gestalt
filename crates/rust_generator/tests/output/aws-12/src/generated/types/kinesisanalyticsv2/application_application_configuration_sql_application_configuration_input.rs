#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationInput {
    #[builder(into)]
    pub r#in_app_stream_names: Option<Vec<String>>,
    #[builder(into)]
    pub r#input_id: Option<String>,
    /// Describes the number of in-application streams to create.
    #[builder(into)]
    pub r#input_parallelism: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputParallelism>>,
    /// The input processing configuration for the input.
    /// An input processor transforms records as they are received from the stream, before the application's SQL code executes.
    #[builder(into)]
    pub r#input_processing_configuration: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputProcessingConfiguration>>,
    /// Describes the format of the data in the streaming source, and how each data element maps to corresponding columns in the in-application stream that is being created.
    #[builder(into)]
    pub r#input_schema: Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputSchema>,
    /// The point at which the application starts processing records from the streaming source.
    #[builder(into)]
    pub r#input_starting_position_configurations: Option<Vec<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputInputStartingPositionConfiguration>>,
    /// If the streaming source is a Kinesis Data Firehose delivery stream, identifies the delivery stream's ARN.
    #[builder(into)]
    pub r#kinesis_firehose_input: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputKinesisFirehoseInput>>,
    /// If the streaming source is a Kinesis data stream, identifies the stream's Amazon Resource Name (ARN).
    #[builder(into)]
    pub r#kinesis_streams_input: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInputKinesisStreamsInput>>,
    /// The name prefix to use when creating an in-application stream.
    #[builder(into)]
    pub r#name_prefix: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationApplicationConfigurationSqlApplicationConfigurationInput {
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
                    "inAppStreamNames",
                    &self.r#in_app_stream_names,
                ),
                to_pulumi_object_field(
                    "inputId",
                    &self.r#input_id,
                ),
                to_pulumi_object_field(
                    "inputParallelism",
                    &self.r#input_parallelism,
                ),
                to_pulumi_object_field(
                    "inputProcessingConfiguration",
                    &self.r#input_processing_configuration,
                ),
                to_pulumi_object_field(
                    "inputSchema",
                    &self.r#input_schema,
                ),
                to_pulumi_object_field(
                    "inputStartingPositionConfigurations",
                    &self.r#input_starting_position_configurations,
                ),
                to_pulumi_object_field(
                    "kinesisFirehoseInput",
                    &self.r#kinesis_firehose_input,
                ),
                to_pulumi_object_field(
                    "kinesisStreamsInput",
                    &self.r#kinesis_streams_input,
                ),
                to_pulumi_object_field(
                    "namePrefix",
                    &self.r#name_prefix,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("inAppStreamNames") {
                            Some(value) => value,
                            None => bail!("Missing field 'inAppStreamNames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_id: {
                        let field_value = match fields_map.get("inputId") {
                            Some(value) => value,
                            None => bail!("Missing field 'inputId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_parallelism: {
                        let field_value = match fields_map.get("inputParallelism") {
                            Some(value) => value,
                            None => bail!("Missing field 'inputParallelism' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_processing_configuration: {
                        let field_value = match fields_map.get("inputProcessingConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'inputProcessingConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_schema: {
                        let field_value = match fields_map.get("inputSchema") {
                            Some(value) => value,
                            None => bail!("Missing field 'inputSchema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#input_starting_position_configurations: {
                        let field_value = match fields_map.get("inputStartingPositionConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'inputStartingPositionConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_firehose_input: {
                        let field_value = match fields_map.get("kinesisFirehoseInput") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesisFirehoseInput' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_streams_input: {
                        let field_value = match fields_map.get("kinesisStreamsInput") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesisStreamsInput' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name_prefix: {
                        let field_value = match fields_map.get("namePrefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'namePrefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
