#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamProcessorOutput {
    /// The Amazon Kinesis Data Streams stream to which the Amazon Rekognition stream processor streams the analysis results. See `kinesis_data_stream`.
    #[builder(into)]
    #[serde(rename = "kinesisDataStream")]
    pub r#kinesis_data_stream: Option<Box<super::super::types::rekognition::StreamProcessorOutputKinesisDataStream>>,
    /// The Amazon S3 bucket location to which Amazon Rekognition publishes the detailed inference results of a video analysis operation. See `s3_destination`.
    #[builder(into)]
    #[serde(rename = "s3Destination")]
    pub r#s_3_destination: Option<Box<super::super::types::rekognition::StreamProcessorOutputS3Destination>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StreamProcessorOutput {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("kinesis_data_stream".to_string(), self.r#kinesis_data_stream.to_pulumi_value().await);
            map.insert("s_3_destination".to_string(), self.r#s_3_destination.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StreamProcessorOutput {
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
                    r#kinesis_data_stream: {
                        let field_value = match fields_map.get("kinesis_data_stream") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_data_stream' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::rekognition::StreamProcessorOutputKinesisDataStream>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_destination: {
                        let field_value = match fields_map.get("s_3_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::rekognition::StreamProcessorOutputS3Destination>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
