#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRealtimeLogConfigEndpoint {
    /// (Required) Amazon Kinesis data stream configuration.
    #[builder(into)]
    #[serde(rename = "kinesisStreamConfigs")]
    pub r#kinesis_stream_configs: Vec<super::super::types::cloudfront::GetRealtimeLogConfigEndpointKinesisStreamConfig>,
    /// (Required) Type of data stream where real-time log data is sent. The only valid value is `Kinesis`.
    #[builder(into)]
    #[serde(rename = "streamType")]
    pub r#stream_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRealtimeLogConfigEndpoint {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("kinesis_stream_configs".to_string(), self.r#kinesis_stream_configs.to_pulumi_value().await);
            map.insert("stream_type".to_string(), self.r#stream_type.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRealtimeLogConfigEndpoint {
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
                    r#kinesis_stream_configs: {
                        let field_value = match fields_map.get("kinesis_stream_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_stream_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::cloudfront::GetRealtimeLogConfigEndpointKinesisStreamConfig> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#stream_type: {
                        let field_value = match fields_map.get("stream_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'stream_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
