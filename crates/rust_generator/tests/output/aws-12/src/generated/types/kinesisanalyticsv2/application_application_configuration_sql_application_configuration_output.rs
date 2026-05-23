#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfigurationOutput {
    /// Describes the data format when records are written to the destination.
    #[builder(into)]
    pub r#destination_schema: Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationOutputDestinationSchema>,
    /// Identifies a Kinesis Data Firehose delivery stream as the destination.
    #[builder(into)]
    pub r#kinesis_firehose_output: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationOutputKinesisFirehoseOutput>>,
    /// Identifies a Kinesis data stream as the destination.
    #[builder(into)]
    pub r#kinesis_streams_output: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationOutputKinesisStreamsOutput>>,
    /// Identifies a Lambda function as the destination.
    #[builder(into)]
    pub r#lambda_output: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationOutputLambdaOutput>>,
    /// The name of the in-application stream.
    #[builder(into)]
    pub r#name: String,
    #[builder(into)]
    pub r#output_id: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationApplicationConfigurationSqlApplicationConfigurationOutput {
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
                    "destinationSchema",
                    &self.r#destination_schema,
                ),
                to_pulumi_object_field(
                    "kinesisFirehoseOutput",
                    &self.r#kinesis_firehose_output,
                ),
                to_pulumi_object_field(
                    "kinesisStreamsOutput",
                    &self.r#kinesis_streams_output,
                ),
                to_pulumi_object_field(
                    "lambdaOutput",
                    &self.r#lambda_output,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "outputId",
                    &self.r#output_id,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("destinationSchema") {
                            Some(value) => value,
                            None => bail!("Missing field 'destinationSchema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_firehose_output: {
                        let field_value = match fields_map.get("kinesisFirehoseOutput") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesisFirehoseOutput' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_streams_output: {
                        let field_value = match fields_map.get("kinesisStreamsOutput") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesisStreamsOutput' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lambda_output: {
                        let field_value = match fields_map.get("lambdaOutput") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambdaOutput' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("outputId") {
                            Some(value) => value,
                            None => bail!("Missing field 'outputId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
