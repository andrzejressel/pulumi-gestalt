#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AnalyticsApplicationOutput {
    /// The ARN of the Kinesis Analytics Application.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// The Kinesis Firehose configuration for the destination stream. Conflicts with `kinesis_stream`.
    /// See Kinesis Firehose below for more details.
    #[builder(into)]
    #[serde(rename = "kinesisFirehose")]
    pub r#kinesis_firehose: Option<Box<super::super::types::kinesis::AnalyticsApplicationOutputKinesisFirehose>>,
    /// The Kinesis Stream configuration for the destination stream. Conflicts with `kinesis_firehose`.
    /// See Kinesis Stream below for more details.
    #[builder(into)]
    #[serde(rename = "kinesisStream")]
    pub r#kinesis_stream: Option<Box<super::super::types::kinesis::AnalyticsApplicationOutputKinesisStream>>,
    /// The Lambda function destination. See Lambda below for more details.
    #[builder(into)]
    #[serde(rename = "lambda")]
    pub r#lambda: Option<Box<super::super::types::kinesis::AnalyticsApplicationOutputLambda>>,
    /// The Name of the in-application stream.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The Schema format of the data written to the destination. See Destination Schema below for more details.
    #[builder(into)]
    #[serde(rename = "schema")]
    pub r#schema: Box<super::super::types::kinesis::AnalyticsApplicationOutputSchema>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AnalyticsApplicationOutput {
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
            map.insert("lambda".to_string(), self.r#lambda.to_pulumi_value().await);
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);
            map.insert("schema".to_string(), self.r#schema.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AnalyticsApplicationOutput {
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
                        <Option<Box<super::super::types::kinesis::AnalyticsApplicationOutputKinesisFirehose>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#kinesis_stream: {
                        let field_value = match fields_map.get("kinesis_stream") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_stream' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesis::AnalyticsApplicationOutputKinesisStream>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#lambda: {
                        let field_value = match fields_map.get("lambda") {
                            Some(value) => value,
                            None => bail!("Missing field 'lambda' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::kinesis::AnalyticsApplicationOutputLambda>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#schema: {
                        let field_value = match fields_map.get("schema") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::kinesis::AnalyticsApplicationOutputSchema> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
