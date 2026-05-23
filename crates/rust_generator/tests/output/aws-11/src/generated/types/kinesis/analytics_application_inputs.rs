#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AnalyticsApplicationInputs {
    /// The ARN of the Kinesis Analytics Application.
    #[builder(into)]
    pub r#id: Option<String>,
    /// The Kinesis Firehose configuration for the streaming source. Conflicts with `kinesis_stream`.
    /// See Kinesis Firehose below for more details.
    #[builder(into)]
    pub r#kinesis_firehose: Option<Box<super::super::types::kinesis::AnalyticsApplicationInputsKinesisFirehose>>,
    /// The Kinesis Stream configuration for the streaming source. Conflicts with `kinesis_firehose`.
    /// See Kinesis Stream below for more details.
    #[builder(into)]
    pub r#kinesis_stream: Option<Box<super::super::types::kinesis::AnalyticsApplicationInputsKinesisStream>>,
    /// The Name Prefix to use when creating an in-application stream.
    #[builder(into)]
    pub r#name_prefix: String,
    /// The number of Parallel in-application streams to create.
    /// See Parallelism below for more details.
    #[builder(into)]
    pub r#parallelism: Option<Box<super::super::types::kinesis::AnalyticsApplicationInputsParallelism>>,
    /// The Processing Configuration to transform records as they are received from the stream.
    /// See Processing Configuration below for more details.
    #[builder(into)]
    pub r#processing_configuration: Option<Box<super::super::types::kinesis::AnalyticsApplicationInputsProcessingConfiguration>>,
    /// The Schema format of the data in the streaming source. See Source Schema below for more details.
    #[builder(into)]
    pub r#schema: Box<super::super::types::kinesis::AnalyticsApplicationInputsSchema>,
    /// The point at which the application starts processing records from the streaming source.
    /// See Starting Position Configuration below for more details.
    #[builder(into)]
    pub r#starting_position_configurations: Option<Vec<super::super::types::kinesis::AnalyticsApplicationInputsStartingPositionConfiguration>>,
    #[builder(into)]
    pub r#stream_names: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AnalyticsApplicationInputs {
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
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "kinesisFirehose",
                    &self.r#kinesis_firehose,
                ),
                to_pulumi_object_field(
                    "kinesisStream",
                    &self.r#kinesis_stream,
                ),
                to_pulumi_object_field(
                    "namePrefix",
                    &self.r#name_prefix,
                ),
                to_pulumi_object_field(
                    "parallelism",
                    &self.r#parallelism,
                ),
                to_pulumi_object_field(
                    "processingConfiguration",
                    &self.r#processing_configuration,
                ),
                to_pulumi_object_field(
                    "schema",
                    &self.r#schema,
                ),
                to_pulumi_object_field(
                    "startingPositionConfigurations",
                    &self.r#starting_position_configurations,
                ),
                to_pulumi_object_field(
                    "streamNames",
                    &self.r#stream_names,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AnalyticsApplicationInputs {
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
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_firehose: {
                        let field_value = match fields_map.get("kinesisFirehose") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesisFirehose' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_stream: {
                        let field_value = match fields_map.get("kinesisStream") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesisStream' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#parallelism: {
                        let field_value = match fields_map.get("parallelism") {
                            Some(value) => value,
                            None => bail!("Missing field 'parallelism' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#processing_configuration: {
                        let field_value = match fields_map.get("processingConfiguration") {
                            Some(value) => value,
                            None => bail!("Missing field 'processingConfiguration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schema: {
                        let field_value = match fields_map.get("schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#starting_position_configurations: {
                        let field_value = match fields_map.get("startingPositionConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'startingPositionConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#stream_names: {
                        let field_value = match fields_map.get("streamNames") {
                            Some(value) => value,
                            None => bail!("Missing field 'streamNames' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
