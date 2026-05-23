#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AnalyticsApplicationOutput {
    /// The ARN of the Kinesis Analytics Application.
    #[builder(into)]
    pub r#id: Option<String>,
    /// The Kinesis Firehose configuration for the destination stream. Conflicts with `kinesis_stream`.
    /// See Kinesis Firehose below for more details.
    #[builder(into)]
    pub r#kinesis_firehose: Option<Box<super::super::types::kinesis::AnalyticsApplicationOutputKinesisFirehose>>,
    /// The Kinesis Stream configuration for the destination stream. Conflicts with `kinesis_firehose`.
    /// See Kinesis Stream below for more details.
    #[builder(into)]
    pub r#kinesis_stream: Option<Box<super::super::types::kinesis::AnalyticsApplicationOutputKinesisStream>>,
    /// The Lambda function destination. See Lambda below for more details.
    #[builder(into)]
    pub r#lambda: Option<Box<super::super::types::kinesis::AnalyticsApplicationOutputLambda>>,
    /// The Name of the in-application stream.
    #[builder(into)]
    pub r#name: String,
    /// The Schema format of the data written to the destination. See Destination Schema below for more details.
    #[builder(into)]
    pub r#schema: Box<super::super::types::kinesis::AnalyticsApplicationOutputSchema>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AnalyticsApplicationOutput {
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
                    "lambda",
                    &self.r#lambda,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "schema",
                    &self.r#schema,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AnalyticsApplicationOutput {
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
                    r#lambda: {
                        let field_value = match fields_map.get("lambda") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#schema: {
                        let field_value = match fields_map.get("schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
