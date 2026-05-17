#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationOutput {
    /// Describes the data format when records are written to the destination.
    #[builder(into)]
    #[serde(rename = "destinationSchema")]
    pub r#destination_schema: Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationOutputDestinationSchema>,
    /// Identifies a Kinesis Data Firehose delivery stream as the destination.
    #[builder(into)]
    #[serde(rename = "kinesisFirehoseOutput")]
    pub r#kinesis_firehose_output: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationOutputKinesisFirehoseOutput>>,
    /// Identifies a Kinesis data stream as the destination.
    #[builder(into)]
    #[serde(rename = "kinesisStreamsOutput")]
    pub r#kinesis_streams_output: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationOutputKinesisStreamsOutput>>,
    /// Identifies a Lambda function as the destination.
    #[builder(into)]
    #[serde(rename = "lambdaOutput")]
    pub r#lambda_output: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationOutputLambdaOutput>>,
    /// The name of the in-application stream.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    #[builder(into)]
    #[serde(rename = "outputId")]
    pub r#output_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationApplicationConfigurationSqlApplicationConfigurationOutput {
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
                "destination_schema".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination_schema,
                )
                .await,
            );
            map.insert(
                "kinesis_firehose_output".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kinesis_firehose_output,
                )
                .await,
            );
            map.insert(
                "kinesis_streams_output".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kinesis_streams_output,
                )
                .await,
            );
            map.insert(
                "lambda_output".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lambda_output,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "output_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#output_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationApplicationConfigurationSqlApplicationConfigurationOutput {
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
                    r#destination_schema: {
                        let field_value = match fields_map.get("destination_schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_firehose_output: {
                        let field_value = match fields_map.get("kinesis_firehose_output") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_firehose_output' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_streams_output: {
                        let field_value = match fields_map.get("kinesis_streams_output") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_streams_output' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lambda_output: {
                        let field_value = match fields_map.get("lambda_output") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda_output' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#output_id: {
                        let field_value = match fields_map.get("output_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
