#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectorLogDeliveryWorkerLogDelivery {
    /// Details about delivering logs to Amazon CloudWatch Logs. See `cloudwatch_logs` Block for details.
    #[builder(into)]
    #[serde(rename = "cloudwatchLogs")]
    pub r#cloudwatch_logs: Option<Box<super::super::types::mskconnect::ConnectorLogDeliveryWorkerLogDeliveryCloudwatchLogs>>,
    /// Details about delivering logs to Amazon Kinesis Data Firehose. See `firehose` Block for details.
    #[builder(into)]
    #[serde(rename = "firehose")]
    pub r#firehose: Option<Box<super::super::types::mskconnect::ConnectorLogDeliveryWorkerLogDeliveryFirehose>>,
    /// Details about delivering logs to Amazon S3. See `s3` Block for deetails.
    #[builder(into)]
    #[serde(rename = "s3")]
    pub r#s_3: Option<Box<super::super::types::mskconnect::ConnectorLogDeliveryWorkerLogDeliveryS3>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectorLogDeliveryWorkerLogDelivery {
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
                "cloudwatch_logs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cloudwatch_logs,
                )
                .await,
            );
            map.insert(
                "firehose".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#firehose,
                )
                .await,
            );
            map.insert(
                "s_3".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectorLogDeliveryWorkerLogDelivery {
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
                    r#cloudwatch_logs: {
                        let field_value = match fields_map.get("cloudwatch_logs") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloudwatch_logs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#firehose: {
                        let field_value = match fields_map.get("firehose") {
                            Some(value) => value,
                            None => bail!("Missing field 'firehose' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3: {
                        let field_value = match fields_map.get("s_3") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
