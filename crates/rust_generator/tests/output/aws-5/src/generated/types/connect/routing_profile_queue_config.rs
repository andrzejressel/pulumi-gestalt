#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RoutingProfileQueueConfig {
    /// Specifies the channels agents can handle in the Contact Control Panel (CCP) for this routing profile. Valid values are `VOICE`, `CHAT`, `TASK`.
    #[builder(into)]
    #[serde(rename = "channel")]
    pub r#channel: String,
    /// Specifies the delay, in seconds, that a contact should be in the queue before they are routed to an available agent
    #[builder(into)]
    #[serde(rename = "delay")]
    pub r#delay: i32,
    /// Specifies the order in which contacts are to be handled for the queue.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// ARN for the queue.
    #[builder(into)]
    #[serde(rename = "queueArn")]
    pub r#queue_arn: Option<String>,
    /// Specifies the identifier for the queue.
    #[builder(into)]
    #[serde(rename = "queueId")]
    pub r#queue_id: String,
    /// Name for the queue.
    #[builder(into)]
    #[serde(rename = "queueName")]
    pub r#queue_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RoutingProfileQueueConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("channel".to_string(), self.r#channel.to_pulumi_value().await);
            map.insert("delay".to_string(), self.r#delay.to_pulumi_value().await);
            map.insert("priority".to_string(), self.r#priority.to_pulumi_value().await);
            map.insert("queue_arn".to_string(), self.r#queue_arn.to_pulumi_value().await);
            map.insert("queue_id".to_string(), self.r#queue_id.to_pulumi_value().await);
            map.insert("queue_name".to_string(), self.r#queue_name.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RoutingProfileQueueConfig {
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
                    r#channel: {
                        let field_value = match fields_map.get("channel") {
                            Some(value) => value,
                            None => bail!("Missing field 'channel' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#delay: {
                        let field_value = match fields_map.get("delay") {
                            Some(value) => value,
                            None => bail!("Missing field 'delay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#priority: {
                        let field_value = match fields_map.get("priority") {
                            Some(value) => value,
                            None => bail!("Missing field 'priority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#queue_arn: {
                        let field_value = match fields_map.get("queue_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'queue_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#queue_id: {
                        let field_value = match fields_map.get("queue_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'queue_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#queue_name: {
                        let field_value = match fields_map.get("queue_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'queue_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
