#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AnalyticsApplicationInputs {
    /// The ARN of the Kinesis Analytics Application.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The Kinesis Firehose configuration for the streaming source. Conflicts with `kinesis_stream`.
    /// See Kinesis Firehose below for more details.
    #[builder(into)]
    #[serde(rename = "kinesisFirehose")]
    pub r#kinesis_firehose: Option<Box<super::super::types::kinesis::AnalyticsApplicationInputsKinesisFirehose>>,
    /// The Kinesis Stream configuration for the streaming source. Conflicts with `kinesis_firehose`.
    /// See Kinesis Stream below for more details.
    #[builder(into)]
    #[serde(rename = "kinesisStream")]
    pub r#kinesis_stream: Option<Box<super::super::types::kinesis::AnalyticsApplicationInputsKinesisStream>>,
    /// The Name Prefix to use when creating an in-application stream.
    #[builder(into)]
    #[serde(rename = "namePrefix")]
    pub r#name_prefix: String,
    /// The number of Parallel in-application streams to create.
    /// See Parallelism below for more details.
    #[builder(into)]
    #[serde(rename = "parallelism")]
    pub r#parallelism: Option<Box<super::super::types::kinesis::AnalyticsApplicationInputsParallelism>>,
    /// The Processing Configuration to transform records as they are received from the stream.
    /// See Processing Configuration below for more details.
    #[builder(into)]
    #[serde(rename = "processingConfiguration")]
    pub r#processing_configuration: Option<Box<super::super::types::kinesis::AnalyticsApplicationInputsProcessingConfiguration>>,
    /// The Schema format of the data in the streaming source. See Source Schema below for more details.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Box<super::super::types::kinesis::AnalyticsApplicationInputsSchema>,
    /// The point at which the application starts processing records from the streaming source.
    /// See Starting Position Configuration below for more details.
    #[builder(into)]
    #[serde(rename = "startingPositionConfigurations")]
    pub r#starting_position_configurations: Option<Vec<super::super::types::kinesis::AnalyticsApplicationInputsStartingPositionConfiguration>>,
    #[builder(into)]
    #[serde(rename = "streamNames")]
    pub r#stream_names: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AnalyticsApplicationInputs {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("id".to_string(), self.r#id.to_pulumi_value().await);
            map.insert("kinesis_firehose".to_string(), self.r#kinesis_firehose.to_pulumi_value().await);
            map.insert("kinesis_stream".to_string(), self.r#kinesis_stream.to_pulumi_value().await);
            map.insert("name_prefix".to_string(), self.r#name_prefix.to_pulumi_value().await);
            map.insert("parallelism".to_string(), self.r#parallelism.to_pulumi_value().await);
            map.insert("processing_configuration".to_string(), self.r#processing_configuration.to_pulumi_value().await);
            map.insert("schema".to_string(), self.r#schema.to_pulumi_value().await);
            map.insert("starting_position_configurations".to_string(), self.r#starting_position_configurations.to_pulumi_value().await);
            map.insert("stream_names".to_string(), self.r#stream_names.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AnalyticsApplicationInputs {
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
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#kinesis_firehose: {
                        let field_value = match fields_map.get("kinesis_firehose") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_firehose' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesis::AnalyticsApplicationInputsKinesisFirehose>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#kinesis_stream: {
                        let field_value = match fields_map.get("kinesis_stream") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_stream' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesis::AnalyticsApplicationInputsKinesisStream>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name_prefix: {
                        let field_value = match fields_map.get("name_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'name_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#parallelism: {
                        let field_value = match fields_map.get("parallelism") {
                            Some(value) => value,
                            None => bail!("Missing field 'parallelism' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesis::AnalyticsApplicationInputsParallelism>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#processing_configuration: {
                        let field_value = match fields_map.get("processing_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'processing_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesis::AnalyticsApplicationInputsProcessingConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#schema: {
                        let field_value = match fields_map.get("schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::kinesis::AnalyticsApplicationInputsSchema> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#starting_position_configurations: {
                        let field_value = match fields_map.get("starting_position_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'starting_position_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::kinesis::AnalyticsApplicationInputsStartingPositionConfiguration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#stream_names: {
                        let field_value = match fields_map.get("stream_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'stream_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
