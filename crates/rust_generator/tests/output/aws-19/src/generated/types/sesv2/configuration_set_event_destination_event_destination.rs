#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigurationSetEventDestinationEventDestination {
    /// An object that defines an Amazon CloudWatch destination for email events. See `cloud_watch_destination` Block for details.
    #[builder(into)]
    #[serde(rename = "cloudWatchDestination")]
    pub r#cloud_watch_destination: Option<Box<super::super::types::sesv2::ConfigurationSetEventDestinationEventDestinationCloudWatchDestination>>,
    /// When the event destination is enabled, the specified event types are sent to the destinations. Default: `false`.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    #[builder(into)]
    #[serde(rename = "eventBridgeDestination")]
    pub r#event_bridge_destination: Option<Box<super::super::types::sesv2::ConfigurationSetEventDestinationEventDestinationEventBridgeDestination>>,
    /// An object that defines an Amazon Kinesis Data Firehose destination for email events. See `kinesis_firehose_destination` Block for details.
    #[builder(into)]
    #[serde(rename = "kinesisFirehoseDestination")]
    pub r#kinesis_firehose_destination: Option<Box<super::super::types::sesv2::ConfigurationSetEventDestinationEventDestinationKinesisFirehoseDestination>>,
    /// An array that specifies which events the Amazon SES API v2 should send to the destinations. Valid values: `SEND`, `REJECT`, `BOUNCE`, `COMPLAINT`, `DELIVERY`, `OPEN`, `CLICK`, `RENDERING_FAILURE`, `DELIVERY_DELAY`, `SUBSCRIPTION`.
    #[builder(into)]
    #[serde(rename = "matchingEventTypes")]
    pub r#matching_event_types: Vec<String>,
    /// An object that defines an Amazon Pinpoint project destination for email events. See `pinpoint_destination` Block for details.
    #[builder(into)]
    #[serde(rename = "pinpointDestination")]
    pub r#pinpoint_destination: Option<Box<super::super::types::sesv2::ConfigurationSetEventDestinationEventDestinationPinpointDestination>>,
    /// An object that defines an Amazon SNS destination for email events. See `sns_destination` Block for details.
    #[builder(into)]
    #[serde(rename = "snsDestination")]
    pub r#sns_destination: Option<Box<super::super::types::sesv2::ConfigurationSetEventDestinationEventDestinationSnsDestination>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConfigurationSetEventDestinationEventDestination {
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
                    "cloud_watch_destination",
                    &self.r#cloud_watch_destination,
                ),
                to_pulumi_object_field(
                    "enabled",
                    &self.r#enabled,
                ),
                to_pulumi_object_field(
                    "event_bridge_destination",
                    &self.r#event_bridge_destination,
                ),
                to_pulumi_object_field(
                    "kinesis_firehose_destination",
                    &self.r#kinesis_firehose_destination,
                ),
                to_pulumi_object_field(
                    "matching_event_types",
                    &self.r#matching_event_types,
                ),
                to_pulumi_object_field(
                    "pinpoint_destination",
                    &self.r#pinpoint_destination,
                ),
                to_pulumi_object_field(
                    "sns_destination",
                    &self.r#sns_destination,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConfigurationSetEventDestinationEventDestination {
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
                    r#cloud_watch_destination: {
                        let field_value = match fields_map.get("cloud_watch_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'cloud_watch_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event_bridge_destination: {
                        let field_value = match fields_map.get("event_bridge_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_bridge_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kinesis_firehose_destination: {
                        let field_value = match fields_map.get("kinesis_firehose_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'kinesis_firehose_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#matching_event_types: {
                        let field_value = match fields_map.get("matching_event_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'matching_event_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pinpoint_destination: {
                        let field_value = match fields_map.get("pinpoint_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'pinpoint_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sns_destination: {
                        let field_value = match fields_map.get("sns_destination") {
                            Some(value) => value,
                            None => bail!("Missing field 'sns_destination' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
