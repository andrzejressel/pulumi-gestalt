#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BucketNotificationTopic {
    /// [Event](http://docs.aws.amazon.com/AmazonS3/latest/dev/NotificationHowTo.html#notification-how-to-event-types-and-destinations) for which to send notifications.
    #[builder(into)]
    #[serde(rename = "events")]
    pub r#events: Vec<String>,
    /// Object key name prefix.
    #[builder(into)]
    #[serde(rename = "filterPrefix")]
    pub r#filter_prefix: Option<String>,
    /// Object key name suffix.
    #[builder(into)]
    #[serde(rename = "filterSuffix")]
    pub r#filter_suffix: Option<String>,
    /// Unique identifier for each of the notification configurations.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// SNS topic ARN.
    #[builder(into)]
    #[serde(rename = "topicArn")]
    pub r#topic_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BucketNotificationTopic {
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
                "events".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#events,
                )
                .await,
            );
            map.insert(
                "filter_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filter_prefix,
                )
                .await,
            );
            map.insert(
                "filter_suffix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#filter_suffix,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "topic_arn".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#topic_arn,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BucketNotificationTopic {
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
                    r#events: {
                        let field_value = match fields_map.get("events") {
                            Some(value) => value,
                            None => bail!("Missing field 'events' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_prefix: {
                        let field_value = match fields_map.get("filter_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filter_suffix: {
                        let field_value = match fields_map.get("filter_suffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'filter_suffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#topic_arn: {
                        let field_value = match fields_map.get("topic_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'topic_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
