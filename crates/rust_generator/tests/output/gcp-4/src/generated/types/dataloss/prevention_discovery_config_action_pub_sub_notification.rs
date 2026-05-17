#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigActionPubSubNotification {
    /// How much data to include in the pub/sub message.
    /// Possible values are: `TABLE_PROFILE`, `RESOURCE_NAME`.
    #[builder(into)]
    #[serde(rename = "detailOfMessage")]
    pub r#detail_of_message: Option<String>,
    /// The type of event that triggers a Pub/Sub. At most one PubSubNotification per EventType is permitted.
    /// Possible values are: `NEW_PROFILE`, `CHANGED_PROFILE`, `SCORE_INCREASED`, `ERROR_CHANGED`.
    #[builder(into)]
    #[serde(rename = "event")]
    pub r#event: Option<String>,
    /// Conditions for triggering pubsub
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pubsubCondition")]
    pub r#pubsub_condition: Option<Box<super::super::types::dataloss::PreventionDiscoveryConfigActionPubSubNotificationPubsubCondition>>,
    /// Cloud Pub/Sub topic to send notifications to. Format is projects/{project}/topics/{topic}.
    #[builder(into)]
    #[serde(rename = "topic")]
    pub r#topic: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDiscoveryConfigActionPubSubNotification {
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
                "detail_of_message".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#detail_of_message,
                )
                .await,
            );
            map.insert(
                "event".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#event,
                )
                .await,
            );
            map.insert(
                "pubsub_condition".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#pubsub_condition,
                )
                .await,
            );
            map.insert(
                "topic".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#topic,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDiscoveryConfigActionPubSubNotification {
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
                    r#detail_of_message: {
                        let field_value = match fields_map.get("detail_of_message") {
                            Some(value) => value,
                            None => bail!("Missing field 'detail_of_message' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event: {
                        let field_value = match fields_map.get("event") {
                            Some(value) => value,
                            None => bail!("Missing field 'event' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pubsub_condition: {
                        let field_value = match fields_map.get("pubsub_condition") {
                            Some(value) => value,
                            None => bail!("Missing field 'pubsub_condition' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topic: {
                        let field_value = match fields_map.get("topic") {
                            Some(value) => value,
                            None => bail!("Missing field 'topic' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
