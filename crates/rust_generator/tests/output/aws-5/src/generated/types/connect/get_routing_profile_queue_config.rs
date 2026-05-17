#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRoutingProfileQueueConfig {
    /// Channels agents can handle in the Contact Control Panel (CCP) for this routing profile. Valid values are `VOICE`, `CHAT`, `TASK`.
    #[builder(into)]
    #[serde(rename = "channel")]
    pub r#channel: String,
    /// Delay, in seconds, that a contact should be in the queue before they are routed to an available agent
    #[builder(into)]
    #[serde(rename = "delay")]
    pub r#delay: i32,
    /// Order in which contacts are to be handled for the queue.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// ARN for the queue.
    #[builder(into)]
    #[serde(rename = "queueArn")]
    pub r#queue_arn: String,
    /// Identifier for the queue.
    #[builder(into)]
    #[serde(rename = "queueId")]
    pub r#queue_id: String,
    /// Name for the queue.
    #[builder(into)]
    #[serde(rename = "queueName")]
    pub r#queue_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRoutingProfileQueueConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "channel",
                    &self.r#channel,
                ),
                to_pulumi_object_field(
                    "delay",
                    &self.r#delay,
                ),
                to_pulumi_object_field(
                    "priority",
                    &self.r#priority,
                ),
                to_pulumi_object_field(
                    "queue_arn",
                    &self.r#queue_arn,
                ),
                to_pulumi_object_field(
                    "queue_id",
                    &self.r#queue_id,
                ),
                to_pulumi_object_field(
                    "queue_name",
                    &self.r#queue_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRoutingProfileQueueConfig {
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
                    r#channel: {
                        let field_value = match fields_map.get("channel") {
                            Some(value) => value,
                            None => bail!("Missing field 'channel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delay: {
                        let field_value = match fields_map.get("delay") {
                            Some(value) => value,
                            None => bail!("Missing field 'delay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#priority: {
                        let field_value = match fields_map.get("priority") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#queue_arn: {
                        let field_value = match fields_map.get("queue_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'queue_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#queue_id: {
                        let field_value = match fields_map.get("queue_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'queue_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#queue_name: {
                        let field_value = match fields_map.get("queue_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'queue_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
