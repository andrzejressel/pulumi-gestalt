#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ReplicationGroupLogDeliveryConfiguration {
    /// Name of either the CloudWatch Logs LogGroup or Kinesis Data Firehose resource.
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: String,
    /// For CloudWatch Logs use `cloudwatch-logs` or for Kinesis Data Firehose use `kinesis-firehose`.
    #[builder(into)]
    #[serde(rename = "destinationType")]
    pub r#destination_type: String,
    /// Valid values are `json` or `text`
    #[builder(into)]
    #[serde(rename = "logFormat")]
    pub r#log_format: String,
    /// Valid values are  `slow-log` or `engine-log`. Max 1 of each.
    #[builder(into)]
    #[serde(rename = "logType")]
    pub r#log_type: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ReplicationGroupLogDeliveryConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "destination".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination,
                )
                .await,
            );
            map.insert(
                "destination_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#destination_type,
                )
                .await,
            );
            map.insert(
                "log_format".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_format,
                )
                .await,
            );
            map.insert(
                "log_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_type,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ReplicationGroupLogDeliveryConfiguration {
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
                    r#destination: {
                        let field_value = match fields_map.get("destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#destination_type: {
                        let field_value = match fields_map.get("destination_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'destination_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_format: {
                        let field_value = match fields_map.get("log_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_type: {
                        let field_value = match fields_map.get("log_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
